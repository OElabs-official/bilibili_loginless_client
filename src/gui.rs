use 
{
    eframe::{egui, epi},
    eframe::egui::{color::*, *},
    std::sync::{Arc,Mutex},
    crate::*,
    crate::dataprocess::Video,
    egui_design::*,
    chrono::prelude as CP,
};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct Gui //必须有输入，所有不需要默认值
{
    dataset : Arc<Mutex<DataSet>>,
    setting : Arc<Mutex<Setting>>,
    dataset_cache : Vec<Video>,
}
impl Gui
{
    pub fn start(dataset : Arc<Mutex<DataSet>>, setting : Arc<Mutex<Setting>>)->Gui
    {
        let dataset_cache;
        {
            let dataset             =   dataset.lock().unwrap();// ! 小心互锁 , 需要验证
            dataset_cache      =   (*dataset).gui_output(setting.clone());
        }
        Gui{dataset,setting,dataset_cache}
    }
}
impl epi::App for Gui
{
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) 
    {epi::set_value(storage, epi::APP_KEY, self);}

    fn name(&self) -> &str {"<OE_Labs> Bilibili Login-less Client V0.3"}

    fn setup(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>, _storage: Option<&dyn epi::Storage>) 
    {
        oelabs_design(ctx);



    }
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) 
    {*self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()}    


    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) 
    { 

        let mut current_time = CP::Local::now().timestamp();          

        egui::SidePanel::left("sidepanel").show
        (ctx, |ui|{
            ui.vertical //水平布局
            (|ui| {
                {
                    if ui.put
                    (
                        egui::Rect::from_min_size(ui.min_rect().min + vec2(0.0, 0.0),vec2(300.0,40.0)),
                        egui::Button::new("刷新")
                    ).clicked() 
                    {       
                        current_time            = CP::Local::now().timestamp();     
                        let dataset             =   self.dataset.lock().unwrap();
                        self.dataset_cache      =   (*dataset).gui_output(self.setting.clone());
                    };
                }   

                ui.put
                (
                    egui::Rect::from_min_size(ui.min_rect().min + vec2(0.0, 50.0),vec2(300.0,40.0)),
                    egui::Button::new("*设定")
                );   

                {
                    if ui.put
                    (
                        egui::Rect::from_min_size(ui.min_rect().min + vec2(0.0, 100.0),vec2(300.0,40.0)),
                        egui::Button::new("退出")
                    )
                    .clicked() 
                    {
                        frame.quit();
                    };    
                }
                
            });


        });

        egui::CentralPanel::default().show
        (ctx, |ui|
            {
                let text_style = TextStyle::Body;
                let row_height = ui.fonts()[text_style].row_height();
                let num_rows = self.dataset_cache.len(); 

                ui.horizontal //水平布局
                (|ui| {
                    ui.selectable_value
                    (
                        &mut 3, 
                        0, 
                        "显示全部"
                    );
                    ui.selectable_value
                    (
                        &mut 4,
                        1,
                        "按时间顺序显示",
                    );
                    ui.selectable_value
                    (
                        &mut 5,
                        2,
                        "分类",
                    );
                });

                ui.label(format!("time : {}",current_time));

                ui.add_space(5.0);
                
                ScrollArea::auto_sized().show_rows
                (ui, row_height, num_rows, |ui, row_range| {
                    for row in row_range 
                    {
                        let (dspl,url)=self.dataset_cache[row].format();
                        let hl = egui::Hyperlink::new(url).text(dspl);
                        ui.add(hl);

                    }
                });
                ui.add_space(5.0);
            }
        );
    }
} 



pub fn ui(dataset : Arc<Mutex<DataSet>>, setting : Arc<Mutex<Setting>>)
{
    let app = crate::Gui::start(dataset,setting);

    let inative_options;                 
    {
        inative_options    =   eframe::NativeOptions
        {
            always_on_top: false,
            decorated: true,//添加窗口装饰（即应用程序周围的框架）取决于操作系统，flase 需要手动绘制
            drag_and_drop_support: true,
            icon_data: None,
            initial_window_size: Some(egui::Vec2::new(1600.0,1200.0)),
            resizable: true,
            transparent: false,
        }
    }
    eframe::run_native(Box::new(app), inative_options);
}









