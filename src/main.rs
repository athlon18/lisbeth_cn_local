use std::{fs, i32, panic, time};
use std::borrow::Cow;
use std::thread::{sleep, yield_now};

use quick_protobuf::{BytesReader, MessageRead, MessageWrite};
use rusqlite::{Connection, Result};

use protos::template::DB;
use protos::template::KVMap;
use protos::template::Language::{Chinese, English};

mod protos;

#[derive(Clone)]
struct Item {
    id: i32,
    chn_name: String,
}

fn main() -> Result<()> {
    let _result = panic::catch_unwind(|| {
        let data_file = "db.s3db";
        let conn = Connection::open(&data_file).expect("没有找到db.s3db! 请在rb根目录下运行!");
        let mut items = Vec::new();
        let items = match conn.prepare("SELECT id, chnname FROM item where chnname is not null and chnname != ''") {
            Ok(mut dd) => {
                let it = dd.query_map([], |row| {
                    Ok(Item {
                        id: row.get(0)?,
                        chn_name: row.get(1)?,
                    })
                }).expect("没有找到db.s3db! 请在rb根目录下运行!");

                for p in it {
                    items.push(p.expect("没有找到db.s3db! 请在rb根目录下运行!"));
                }
                items
            }
            Err(_) => {
                println!("没有找到db.s3db! 请在rb根目录下运行!");
                return;
            }
        };

        let bin = match fs::read("BotBases/Lisbeth/db.bin") {
            Ok(bin) => {
                fs::write("BotBases/Lisbeth/db.bin.bak",&bin).expect("备份失败");
                bin
            },
            Err(_) => {
                println!("没有找到BotBases/Lisbeth/db.bin文件! 请在rb根目录下运行!或者安装Lisbeth! ");
                return;
            }
        };
        let mut reader = BytesReader::from_bytes(&bin);
        let mut res = DB::from_reader(&mut reader, &bin).expect("读取数据源失败！");
        let mut en = vec![KVMap::new()];
        let mut cn = vec![KVMap::new()];
        let mut no_cn = KVMap::new();
        let mut add = KVMap::new();

        for translation_map in res.TranslationMaps.iter() {
            let tt = translation_map.clone();
            if tt.Language == English {
                en = tt.KeyToName.to_owned();
            } else if translation_map.Language == Chinese {
                cn = tt.KeyToName.to_owned();
                // println!("{:?}", tt.KeyToName)
            }
        }
        let mut cnn = KVMap::new();
        for i in cn.into_iter().flatten() {
            cnn.insert(i.0, i.1);
        }

        for i in en.into_iter().flatten() {
            if cnn.get(&i.0).is_none() {
                no_cn.entry(i.0).or_insert(i.1);
            }
        }
        println!("英文存在，中文没有的数量：{:?}", no_cn.len());

        for item in items.iter() {
            let k = Cow::from("Item.".to_owned() + &item.id.to_string());
            let map = no_cn.get(&*k);
            if !map.is_none() {
                add.entry(k).or_insert(Cow::from(&item.chn_name));
            }
        }
        //println!("{:?}", add.len());

        for item in res.TranslationMaps.iter_mut() {
            if item.Language == Chinese {
                // item.KeyToName = &new_context
                for i in add.iter() {
                    let mut newss = KVMap::new();
                    let sss: String = i.1.trim().to_string() + "";
                    newss.insert(Cow::from(i.0.trim_matches('\t').trim()), Cow::from(sss));
                    item.KeyToName.append(vec![newss].as_mut())
                }
            }
        }

        res.write_file("BotBases/Lisbeth/db.bin").expect("没有找到BotBases/Lisbeth/db.bin文件! 请在rb根目录下运行!或者安装Lisbeth!");
        println!("汉化成功! 此次新增物品汉化{:?}个! ", add.len());
    });
    // println!("{:?}",result);
    loop {
        sleep(time::Duration::from_millis(1000));
        //println!("{:?}",233);
        yield_now();
    }
}
