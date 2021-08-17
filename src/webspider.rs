use
{
    //simple_stopwatch::Stopwatch as SW,
    std::
    {
        sync::{Arc,Mutex},
        collections::VecDeque,
        thread::sleep,
        time::Duration
    },
    crate::{Setting,DataSet},
    //json::JsonValue as JV,
    chrono::prelude as CP,
};
const URLA:&'static str         =   r"https://api.bilibili.com/x/space/arc/search?mid=";
const URLB:&'static str         =   r"&pn=1&ps=25&index=1&jsonp=jsonp";



impl Setting
{
    pub async fn connect(&self) ->  DataSet 
    {
        println!("start connect !");
        let itime                   =   CP::Local::now().timestamp();
        let vid                     =   self.get_targets().0;
        let vurl                    =   up_url_wrapper(&vid);
        let vjsn                    =   task_controler(vurl).await;
        let output                  =   DataSet::new(vjsn,itime,vid);
        let _ = std::fs::write(crate::init::PTH_LAST, itime.to_string());   //记录同步时间        
        output.save();       //保存缓存
        output
    }
}

fn up_url_wrapper(vid:&Vec<String>)    ->  VecDeque<String>
{
    vid.iter().map(|x|(format!("{}{}{}",URLA,x,URLB))).collect::<VecDeque<_>>()
}
fn new_client() -> reqwest::Client
{
    let  clientbuild0               =   reqwest::ClientBuilder::new()
    .user_agent(r"OElabs 2021-Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36/");
    clientbuild0.build().unwrap()  
}


async fn task_controler(vurl: VecDeque<String>) ->Vec<json::JsonValue>//改回vec
{
    let client0                     =  Arc::new(new_client()); 
        let mut vtsk                =   vec![];
        for i0 in 0..vurl.len()
        {   
            let iclient             =   client0.clone();
            let iurl                =   vurl[i0].clone();
            vtsk.push
            (
                tokio::spawn(async move  
                {
                    let irqst       =   iclient.get(iurl.as_str()).build().unwrap();
                    let ibody       =   iclient.execute(irqst).await.unwrap().text().await.unwrap();
                    let jsn         =   json::parse(ibody.as_str()).unwrap();
                    jsn
                })
            )
        }
        let mut output              =   vec![];
        for i0 in vtsk
        {
            output.push(i0.await.unwrap_or(json::JsonValue::new_object()));
        }
        output
}


pub fn spider_alive// thread func, (Setting , Dataset) shared with Arc && Mutex
    (
        next_time   :u64,
        setting     :Arc<Mutex<Setting>>,
        dataset     :Arc<Mutex<DataSet>>
    )
{
    let max_thread;
    { 
        let ptr = setting.lock().unwrap();
        max_thread  =   (*ptr).max_thread;
    }
    
    let  tkrt;
    {
        let mut mt                  =   tokio::runtime::Builder::new_multi_thread();
        if max_thread >1 
        {
            tkrt                    =   mt.worker_threads(max_thread).enable_all().build().unwrap();
            println!("Inf : tokio runtime start, max thread = {}",max_thread);
        }
        else             
        {
            tkrt                    =   mt.enable_all().build().unwrap();
            println!("Inf : tokio runtime start, enable all-core");
        }
    }
    tkrt.block_on
    (async
    {
        let update_period;
        {
            let lcd_setting         =    setting.lock().unwrap();
            update_period           =   (*lcd_setting).get_update_period_sec() as u64;
        }

        sleep(Duration::from_secs(next_time));
        {
            let idataset            =   (*setting.lock().unwrap()).connect().await;
            *dataset.lock().unwrap()=   idataset;
        }
        loop//start loop connect
        {
            sleep(Duration::from_secs(update_period));
            let idataset            =   (*setting.lock().unwrap()).connect().await;
            *dataset.lock().unwrap()=   idataset;
        }
    }
    );
}

