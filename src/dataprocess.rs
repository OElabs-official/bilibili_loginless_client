use 
{
    crate::{DataSet,Setting},
    json::JsonValue as JV,
    std::sync::{Arc,Mutex},
    chrono::prelude as CP,
};
const VIDEO_URL : &'static str       =   r"https://www.bilibili.com/video/";
const PTH_CACHE : &'static str       =   r"cache.json";

pub struct Video
{
    time_to_now : i64,
    title : String,
    url : String,
    dspl_name : String,
    class : String
}
impl Video
{
    fn new
    (   
        time_to_now : i64,
        title : String,
        url : String,
        dspl_name : String,
        class : String
    )   ->Self
    {
        Self{time_to_now,title,url,dspl_name,class}
    }
    pub fn disp_time(time_to_now:i64)->String //把时间戳的差值转换为字符串
    {
        if time_to_now < 60*60
        {
            format!("{:>6}分钟前",time_to_now /60)
        }
        else if time_to_now < 60*60*24
        {
            format!("{:>6}小时前",time_to_now /3600)
        }
        else 
        {
            format!("{:>6} 天前",time_to_now /3600/24)
        }     
    }
    pub fn format(&self) -> (String,String) // dspl,url
    {
        let url = self.url.clone();
        let dspl;
        {
            dspl = format!
            (
                "<{}>\t{}\t{}",self.dspl_name,self.title,Video::disp_time(self.time_to_now)
            );
        }
        (dspl,url)
    }
}
impl DataSet
{
    pub fn save(&self)
    {
        let mut js0                 =   json::JsonValue::new_array();
        for i0 in self.data.iter()
        {
            let _   =   js0.push(i0.clone());
        }
        let _ = std::fs::write(PTH_CACHE, format!("{}",js0));
    }
    pub fn load(set : &Setting) -> Result<DataSet,Box<dyn std::error::Error>>
    {
        let pkg                     =   json::parse(std::fs::read_to_string(PTH_CACHE)?.as_str())?;
        let mut data:   Vec<JV>     =   vec![];
        for i0 in 0..pkg.len()
        {
            data.push(pkg[i0].clone())
        }
        let time                    =   std::fs::read_to_string(crate::init::PTH_LAST)?.parse::<i64>()?;
        let vid                     =   set.get_targets().0;
        Ok(DataSet{data,time,vid})
    }
    pub fn new(data:Vec<JV>,time:i64,vid:Vec<String>)    -> DataSet //考虑检测长度是否相等
    {
        DataSet{data,time,vid}
    }
    pub fn new_empty()->DataSet
    {
        DataSet{data:vec![],time:0,vid:vec![]}
    }
}
impl DataSet
{
    pub fn dspl_all(&self)
    {
        // let vname                   =   &self.vname;
        // println!("{:?}",vname);
        let n0                      =   self.data.len();
        for i0 in 0..n0
        {
            let ivlist              =   &self.data[i0]["data"]["list"]["vlist"];
            // let mut iname           =   vname[i0].clone();
            for i1 in 0..ivlist.len()
            {
                
                let itime0:i64                      =   format!("{}",ivlist[i1]["created"]).as_str().parse().unwrap();
                let time_to_now :i64                =   self.time   -   itime0;
                let mut disp_time                   =   String::new();              
                {
                    if time_to_now < 60*60
                    {
                        disp_time                       =  format!("{:>6}分钟前\t",time_to_now /60)
                    }
                    else if time_to_now < 60*60*24
                    {
                        disp_time                       =  format!("{:>6}小时前\t",time_to_now /3600)
                    }
                    else 
                    {
                        disp_time                       =  format!("{:>6} 天前\t",time_to_now /3600/24)
                    }        
                }
                println!
                (
                    "\t{0}  {2}{3}\t{1}",
                    disp_time,
                    ivlist[i1]["title"],
                    VIDEO_URL,ivlist[i1]["bvid"],
                    // iname
                ) ;
                // iname                               =   String::new()             
            }
        }

    }
    pub fn export_vname(&self) -> Vec<String>
    {
        self.data.iter().map(|x|format!("{}",x["data"]["list"]["vlist"][0]["author"])).collect::<Vec<_>>()
    }

    pub fn gui_output(&self,setting : Arc<Mutex<Setting>>)  -> Vec<Video>   //需要修改成 按id 分组的输出
    {   
        let time_now:i64                  =   CP::Local::now().timestamp();
        let ( _vid, vclass, vname);
        {
            let mut ptr_setting         =   setting.lock().unwrap();
            if (*ptr_setting).name_check()
            {
                (*ptr_setting).update_names(self.export_vname());
                (*ptr_setting).save()
            }
            let tmp                     =   (*ptr_setting).get_targets();
            {_vid=tmp.0;vclass=tmp.1;vname=tmp.2}
        }
        let mut output = vec![];
        {
            let n0                      =   self.data.len(); //json集合的数量
            for i0 in 0..n0
            {
                let ivlist              =   &self.data[i0]["data"]["list"]["vlist"];//运行时获取视频列表

                for i1 in 0..ivlist.len()
                {
                    let (time_to_now,title,url,dspl_name,class);
                    dspl_name           =   vname[i0].clone();
                    class               =   vclass[i0].clone();
                    {
                        let itime0:i64  =   format!("{}",ivlist[i1]["created"]).as_str().parse().unwrap();
                        time_to_now     =   time_now   -   itime0                        
                    }
                    {
                        title           =   format!("{}",ivlist[i1]["title"])
                    }
                    {
                        url             =   format!("{}{}",VIDEO_URL,ivlist[i1]["bvid"])
                    }
                    output.push(Video::new(time_to_now, title, url, dspl_name, class))
                }
            }
        }
        output
    }
}