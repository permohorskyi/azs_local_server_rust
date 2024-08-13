use serde::{Serialize, Deserialize, Serializer};
use std::collections::{HashMap, BTreeMap};

use std::thread::current;
use async_std::sync::{Arc};
use once_cell::sync::Lazy;
use serde::ser::SerializeMap;
use serde_json::{json, Value,Map};

use uuid::Uuid;
use tokio::sync::Mutex;
// Структура для одного логу
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Log {
    number: u64,
    data: String,
    message: String,
}

// Оновлена структура для зберігання логів, яка тепер може включати вкладені групи
#[derive(Deserialize, Clone, Debug)]
pub enum LogItem {
    SingleLog(Log),
    LogGroup(BTreeMap<String, Vec<LogItem>>),
}
// Реалізація Serialize для LogItem
impl Serialize for LogItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match *self {
            LogItem::SingleLog(ref log) => {
                // Якщо це SingleLog, серіалізуємо лише вміст Log
                log.serialize(serializer)
            },
            LogItem::LogGroup(ref map) => {
                // Якщо це LogGroup, серіалізуємо як map
                let mut map_ser = serializer.serialize_map(Some(map.len()))?;
                for (key, value) in map {
                    map_ser.serialize_entry(key, value)?;
                }
                map_ser.end()
            },
        }
    }
}
// Оновлена структура для зберігання всіх логів і синхронізації
#[derive(Clone, Debug)]
pub struct LogManager {
    logs: Arc<Mutex<Value>>, // Оновлено для вкладених груп
}
static GLOBAL_COUNTER: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::new(Mutex::new(0)));
impl LogManager {
    // Функція для ініціалізації нового LogManager
    pub async fn new() -> Self {
        let value=json!({"logs":[]});
        let log=LogManager {
            logs: Arc::new(Mutex::new(value))
        };
        log
    }
    pub async fn get_log(&self,group_path:Vec<String>) -> String{

        let mut result=String::new();
        let mut logs = self.logs.lock().await;
        let mut current_val: &mut Value = &mut *logs;
        current_val=&mut current_val["logs"];
        let mut index=0;
        let mut find=false;
        for key in &group_path {
            index=0;
            find=false;
            if current_val.is_array(){

                if let Some(arr) = current_val.as_array_mut() {
                    for i in 0..arr.len(){
                        if let Some(value) = arr[i].as_object_mut().unwrap().get_mut(key.as_str()) {
                            if value.is_array() {
                                index=i;
                                find=true;
                                break;
                            }
                        }

                    }
                }
                if find==true {
                    current_val = current_val.get_mut(index).unwrap();
                    current_val = &mut current_val[key.clone()];
                }else{
                    return result;
                }
            }
        }
        let mut arr=current_val.as_array_mut().unwrap();
        for i in 0..arr.len(){
            if let Some(value) = arr[i].as_object_mut() {
                if value.len()==1 {
                    println!("INDEX: {} GROUP: TRUE",i);
                }else{
                    println!("INDEX: {} GROUP: FALSE",i);
                }
            }

        }

        result
    }
    // Асинхронна функція для додавання нового логу
    pub async fn add_log(&self, group_path: Vec<String>, data: String, message: String) {

        let mut counter = GLOBAL_COUNTER.lock().await;
        let mut logs = self.logs.lock().await;

        let log = Log {
            number: *counter,
            data,
            message,
        };

        let mut current_val: &mut Value = &mut *logs;
        current_val=&mut current_val["logs"];
        for key in &group_path {
            let mut find=false;
            let mut index:usize=0;
            if current_val.is_array(){

                if let Some(arr) = current_val.as_array_mut() {
                    for i in 0..arr.len(){
                        if let Some(value) = arr[i].as_object_mut().unwrap().get_mut(key.as_str()) {
                            if value.is_array() {
                                index=i;
                                find=true;
                                break;
                            }
                        }

                    }
                    if find==false{
                        arr.push(json!({key.clone():[]}));
                        index=arr.len()-1;
                    }
                }
                current_val=current_val.get_mut(index).unwrap();
                current_val=&mut current_val[key.clone()];
            }

        }
        //println!("VALUE: {}",serde_json::to_string(current_val).unwrap());
        if let Some(arr) = current_val.as_array_mut() {
            arr.push(serde_json::to_value(log).unwrap());
        } else {
            println!("Expected current_val to be an array, but it's not.");
        }

        *counter += 1;

    }

    // Функція для отримання всіх логів в JSON форматі
    pub async fn get_logs_json(&self) -> String {
        let logs = self.logs.lock().await;
        serde_json::to_string(&*logs).unwrap()
    }
    // pub async fn get_key_json(&self, group_path: Vec<String>)->Value{
    //     // let mut logs = self.logs.lock().await;
    //     // let mut current_group = &mut *logs;
    //     // for i in 0..group_path.len()-1 {
    //     //     let group = current_group.entry(group_path[i].clone()).or_insert_with(|| Vec::new());
    //     //     current_group = if let Some(LogItem::LogGroup(sub_group)) = group.last_mut() {
    //     //         sub_group
    //     //     } else {
    //     //         unreachable!()
    //     //
    //     //     };
    //     //
    //     // }
    //     // serde_json::to_value(current_group).unwrap()
    // }
}
