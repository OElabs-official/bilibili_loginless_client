    /*  文件树

    #必须/第一次启动生成

        setting.json    :   配置文件

    #动态生成

        <cache> :   网页缓存
        lastsync    :   上次同步的时间戳
        log.json    :   处理完的数据,也可以被firefox 访问

    #一次性使用文件

        Uplist.txt  :   使用后转换为 _Uplist.txt
    */
use
{
    crate::{Setting,Target,Status}
};

pub const PTH_SETTING : & 'static str           =   r"./setting.json";
pub const PTH_UPLIST : & 'static str            =   "./Uplist.txt";
pub const PTH_LAST : & 'static str              =   "./last";

impl Target //read,new,save
{
    fn to_js(&self) -> json::JsonValue
    {
        let mut output;   
        {
            output                              =   json::JsonValue::new_object();
            output.insert("id", self.id.clone()).unwrap();
            output.insert("class",match &self.class{Some(s)=>s.as_str(),None=>"None"}).unwrap();
            output.insert("name", match &self.name{Some(s)=>s.as_str(),None=>"None"}).unwrap();
            output.insert("nick_name",  match &self.nick_name{Some(s)=>s.as_str(),None=>"None"}).unwrap();            
        }
        output
    }
    fn new(id: & String) -> Target
    {
        Target{id:id.clone(),class:None,name:None,nick_name:None}
    }
    fn from_js(js:&json::JsonValue) -> Target
    {
        let id;// id check
        {
            id                                  =   format!("{}",js["id"]);
            match id.parse::<u32>()
            {
                Ok(_)=>{},
                Err(_)=>{println!("warning : illegal id :{}",id)}
            }
        }
        let class                               =   match format!("{}",js["class"]).as_str(){"None"=>None,s=>Some(s.to_string())};
        let name                                =   match format!("{}",js["name"]).as_str(){"None"=>None,s=>Some(s.to_string())};
        let nick_name                           =   match format!("{}",js["nick_name"]).as_str(){"None"=>None,s=>Some(s.to_string())};
        Target
        {
            id:format!("{}",js["id"]),
            class,
            name,
            nick_name,
        }
    }
    fn from_js_arr(js:&json::JsonValue) -> Vec<Target>
    {
        if js.len()==0{vec![]}
        else
        {
            let mut output                      =   vec![];
            for i0 in 0..js.len()
            {
                output.push(Target::from_js(&js[i0]));
            }
        output
        }
    } 
    fn targets_from_disk() -> Vec<Target>
    {
        let mut vs0                             =   vec![];
        {
            match std::fs::read_to_string(PTH_UPLIST)
            {
                Ok(r)=>
                {
                    match std::fs::rename(PTH_UPLIST, "./_Uplist.txt"){Err(e)=>{println!("rename fail!:{}",e)},_=>{}};
                    let _  =   r.split("\n")
                    .map(|x0|{match x0.parse::<u32>()
                        {
                            Ok(n)=>
                            {
                                if !vs0.contains(&x0.to_string())
                                {vs0.push(n.to_string())}
                            }
                            Err(_)=>{}
                        }
                    }).collect::<Vec<_>>();
                    
                },
                Err(_)=>{}
            }
        }

        vs0.iter().map(|x0|Target::new(x0)).collect::<Vec<_>>()
    }  
}

impl Setting //read,new,save
{
    fn new() -> Setting 
    {
        let mut targets                         =   vec![];
        let mut new_targets                     =   Target::targets_from_disk();
        if new_targets.len()>0{targets.append(&mut new_targets)}
        Setting
        {
            update_period:2.0,
            display_time_range:36.0,
            max_thread:24,
            display_last:true,
            block_key_words:vec![],
            targets
        }
    }
    fn load() -> Result<Setting, Box<dyn std::error::Error>>
    {
        let s0                                  =   std::fs::read_to_string(PTH_SETTING)?;
        let output;
        {
            let js0                             =   json::parse(s0.as_str())?;
            let update_period                   =   js0["update_period"].as_f64().unwrap_or(2.0);
            let display_time_range              =   js0["display_time_range"].as_f64().unwrap_or(36.0);
            let max_thread                      =   js0["max_thread"].as_usize().unwrap_or(32);
            let display_last                    =   js0["display_last"].as_bool().unwrap_or(true);
            let mut block_key_words             =   vec![];                   
            {
                for i0 in 0..js0["block_key_words"].len()
                {
                    block_key_words.push(format!("{}",i0));
                }
            }
            let mut targets                     =   Target::from_js_arr(& js0["targets"]);  
            let mut new_targets;
            let bsave:bool;
            {
                new_targets                     =   Target::targets_from_disk();
                if new_targets.len()>0{targets.append(&mut new_targets);bsave=true;println!("Inf : new id added!")}else{bsave=false}
            }
            output                              =   Setting{update_period,display_last,display_time_range,block_key_words,targets,max_thread};
            if bsave==true{output.save()}
        }
        Ok(output)
    }
    pub fn save(&self)  
    {
        let mut js0;
        {
            js0                                 =   json::JsonValue::new_object();
            js0.insert("update_period",self.update_period).unwrap();
            js0.insert("display_time_range",self.display_time_range).unwrap();
            js0.insert("display_last",self.display_last).unwrap();
            js0.insert("max_thread",self.max_thread).unwrap();
            let mut kwds;  
            {
                kwds                                =   json::JsonValue::new_array();
                for i0 in self.block_key_words.iter(){let _=kwds.push(i0.as_str());}
            }
            js0.insert("block_key_words",kwds).unwrap();
            js0.insert("targets", json::JsonValue::new_array()).unwrap();
            let _ = self.targets.iter().map(|x0|{let _ = js0["targets"].push(x0.to_js());}).collect::<Vec<_>>();
        }
        match std::fs::write(PTH_SETTING, format!("{}",js0)){Ok(_)=>{println!("Inf : setting save success!")},Err(e)=>{println!("Err : save setting >>> {}",e)}} // Gui 中需要把信息导出
    }
    pub fn _show(&self)  //,debug
    {
        println!("{:?}",self);
    }
    fn append_targets(&mut self,mut vtarget:Vec<Target>)
    {
        self.targets.append(&mut vtarget);
        self.save()
    } 
}


pub fn init() -> (Setting,Status)
{
    let mut status;
    let mut setting: Setting;
    {
        setting                                 =   match Setting::load()//读取配置文件
        {
            Ok(s)=>
            {
                match std::fs::read_to_string(PTH_LAST)//检查上次联网时间
                {
                    Ok(n)=>
                    {
                        match n.parse::<i64>(){Ok(n)=>{status=Status::LastConnect(n)},Err(_)=>{status=Status::Unconnect}}
                    }
                    Err(_)=>{status=Status::Unconnect}
                };                
                s
            },
            Err(e)=>{println!("Err:{},init new ...",e);status=Status::FirstBoot;let s=Setting::new();s.save();s}
        };//setting.show()
    }

    let new_targets;
    {
        new_targets                     =   Target::targets_from_disk();
        if new_targets.len()>0{setting.append_targets(new_targets);println!("Inf : new id added!");status=Status::Unconnect}
    }//检查是否有新id
    
    (setting,status)
}
