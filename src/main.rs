#![windows_subsystem="windows"] //在windows 里关闭cli

use
{
    std::
    {
        sync::{Arc,Mutex},
        thread::sleep,
        time::Duration,        
    },
    simple_stopwatch::Stopwatch as SW,
    json::JsonValue as JV,
    chrono::prelude as CP,
    crate::gui::*,
};

mod init;
mod webspider;
mod dataprocess;
mod gui;

#[derive(Debug)]
struct Target
{
    id:String,
    class:Option<String>,//预设{None,Silent}
    name:Option<String>,//联网获取{None}
    nick_name:Option<String>,//手动填写 ,当可用时使用此名字,否则使用name
}

impl Target //change value
{
    pub fn change_name(&mut self,name:&str)
    {
        self.name   =   Some(name.to_string());
    }
    pub fn change_class(&mut self,class:&str)
    {
        self.class   =   Some(class.to_string());
    }
    pub fn change_nick_name(&mut self,name:&str)
    {
        self.nick_name   =   Some(name.to_string());
    }
    pub fn shadow(&mut self)
    {
        self.class   =   Some("Silent".to_string());
    }
}       
impl Target //get value !
{
    
}

#[derive(Debug)]
pub struct Setting
{
    update_period:f64,
    display_time_range:f64,
    max_thread:usize,
    display_last:bool,
    block_key_words:Vec<String>,
    targets:Vec<Target>
}
impl Setting //change value
{
    pub fn _remove_dup_tgt(){}
    pub fn update_names(&mut self,vname:Vec<String>)//可能要考虑如果名字就是None的情况
    {
        let range   =   self.targets.len();
        for i0 in 0..range
        {
            self.targets[i0].change_name(vname[i0].as_str())
        }
    }
}
impl Setting //get value
{
    pub fn export_targets(&self)//save to disk
    {
        let mut s0                                      =   String::new();
        {
            let _                                       =   self.targets.iter()
            .map(
                |x0|
                {s0.push_str(x0.id.as_str());s0.push_str("\n")}
            ).collect::<Vec<_>>();
        }
        if s0.len()>0
        {   match std::fs::write(init::PTH_UPLIST, s0)
            {
                Ok(_)=>println!("Uplist exported!"),
                Err(e)=>println!("Err: exported fail >>> {}",e)
            }
        }
    }
    pub fn _export_targets_csv(&self)
    {}
    pub fn get_targets(&self) -> (Vec<String>,Vec<String>,Vec<String>)// -> id, class, name
    {
        let (mut vid, mut vclass, mut vname)            = (vec![],vec![],vec![]);
        {
            let _ = self.targets.iter()
            .map(|x0|
            {
                vid.push(x0.id.clone());
                vclass.push(x0.class.clone().unwrap_or("None".to_string()));
                match &x0.nick_name
                {
                    Some(s)=>vname.push(s.clone()),
                    None=>match &x0.name{Some(s)=>vname.push(s.clone()),None=>vname.push(x0.id.clone())}
                }
            }).collect::<Vec<_>>();            
        }
        (vid,vclass,vname)
    }
    pub fn get_update_period_sec(&self)->i64
    {
        (self.update_period*60.0*60.0) as i64
    }
}


impl Setting //self check & update 的
{
    fn self_check(&mut self,status:Status)  -> (DataSet,i64) //仅仅做检查,不会联网
    {
        let  dataset0;        
        /*
        to do 去除重复的id
        */
        let next_time;
        let now_time                                            =  CP::Local::now().timestamp() ;
        match status
        {
            Status::LastConnect(n)=>
            {           
                match DataSet::load(&self)  //读取缓存
                {
                    Ok(d)=>
                    {
                        if now_time-n > 0
                        {
                            let n0     =   self.get_update_period_sec()-(now_time-n);
                            if n0 > 0 {next_time = n0}else {next_time=0}
                        }else {next_time =self.get_update_period_sec()-(now_time-n)}
                        dataset0=d; 
                    }
                    Err(_)=>{next_time=0;dataset0=DataSet::new_empty()} //立即更新,暂时返回一个空数据集
                }         
            }
            __ => 
            {
                next_time=0;dataset0=DataSet::new_empty()
            }
        }
        (dataset0,next_time)
    }
    fn name_check(&mut self) -> bool  //当需要展示dataset 数据的时候,条用此方法
    {
        let mut unkwn_name                              =   false;
        let vname                                       =   self.get_targets().2;
        for i0 in vname
        {
            if i0.as_str() == "None"
            {   
                // self.update_names((*dataset0.lock().unwrap()).export_vname());
                unkwn_name = true;
                break
            }
        }   
        unkwn_name                   
    }
}


pub struct DataSet
{
    data    :   Vec<JV>,
    time    :   i64,
    vid     :   Vec<String>,
    // vclass  :   Vec<String>,
    // vname   :   Vec<String>
}

pub enum Status
{
    FirstBoot,
    Unconnect,
    LastConnect(i64),
}


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() 
{

    let (mut setting,status)                            =   init::init();
    let (dataset0,next_time)                            =   setting.self_check(status);
    let setting                                         =   Arc::new(Mutex::new(setting));
    let dataset0                                        =   Arc::new(Mutex::new(dataset0));
    let _backend_server;
    {
        let ptr_dataset                                 =   dataset0.clone();
        let ptr_setting                                 =   setting.clone();
        _backend_server                                  =   std::thread::spawn(move || 
        {
            crate::webspider::spider_alive(next_time as u64,ptr_setting,ptr_dataset)
        })
    }

    
    crate::gui::ui(dataset0,setting)

    // loop // >> open gui
    // {
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    //     (*dataset0.lock().unwrap()).dspl_all();

    // }
}


/* to do

main thread : 加载完成之后主线程负责gui等处理
2nd thread :  tokio 运行时总线程

dataset 转为arc + mutex

使用mutex/rwlock ，显示作为一个单独的线程，也可以给gui 做准备

json 解析在未来考虑换性能更好的包


*/