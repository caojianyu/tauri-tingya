#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Music {
    author: String,
    cover: String,
    music_name: String,
    src: String,
    describe: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Category {
    id: i32,
    name: String,
}

#[get("/")]
fn list() -> Json<Vec<Music>> {
    let mut list: Vec<Music> = Vec::new();
    list.push(Music {
        author: String::from("买辣椒也用券"),
        cover: String::from("https://gimg2.baidu.com/image_search/src=http%3A%2F%2Fwsingbssdl.kugou.com%2F1b8f4fb47dbb4420758f6013c5f9cad6.jpg&refer=http%3A%2F%2Fwsingbssdl.kugou.com&app=2002&size=f9999,10000&q=a80&n=0&g=0n&fmt=auto?sec=1659667795&t=b9acb4c20fa8dc539923bd54d94d6ba4"),
        music_name: String::from("起风了"),
        src: String::from("https://www.ytmp3.cn/down/75687.mp3"),
        describe: String::from("《起风了》原曲改编自日本歌曲《ヤキモチ》"),
    });
    list.push(Music {
        author: String::from("岑宁儿"),
        cover: String::from("https://img0.baidu.com/it/u=2687900188,1974661185&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=500"),
        music_name: String::from("追光者"),
        src: String::from("https://freetyst.nf.migu.cn/public%2Fproduct4th%2Fproduct36%2F2019%2F09%2F0918%2F2018%E5%B9%B412%E6%9C%8810%E6%97%A510%E7%82%B908%E5%88%86%E5%86%85%E5%AE%B9%E5%87%86%E5%85%A5%E6%A2%A6%E5%93%8D%E5%BC%BA%E9%9F%B350%E9%A6%96%2F%E5%85%A8%E6%9B%B2%E8%AF%95%E5%90%AC%2FMp3_64_22_16%2F6404689Z0K4.mp3?Key=a8c7d2517ee98ba4&Tim=1657006779808&channelid=01&msisdn=4593dc66dedc4ecf9570b343d06fc7f2"),
        describe: String::from("电视剧《夏至未至》的插曲"),
    });
    list.push(Music {
        author: String::from("Tez Cadey"),
        cover: String::from("https://img4.kuwo.cn/star/albumcover/500/13/46/4230134618.jpg"),
        music_name: String::from("Seve"),
        src: String::from("https://www.ytmp3.cn/down/50971.mp3"),
        describe: String::from("Tez Cadey于2015年1月26日发行单曲<Seve>。"),
    });
    list.push(Music {
        author: String::from("Deepmaniak"),
        cover: String::from("https://img2.baidu.com/it/u=3019700041,4049630250&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=313"),
        music_name: String::from("Just Like This"),
        src: String::from("https://www.ytmp3.cn/down/49744.mp3"),
        describe: String::from("改版自美国烟鬼组合、Coldplay制作的歌曲《Something Just Like This》"),
    });
    list.push(Music {
        author: String::from("Lucky Twice"),
        cover: String::from(
            "https://p3.music.126.net/rN96t9wDhoDr1-paQmI3mA==/3272146604606179.jpg?param=130y130",
        ),
        music_name: String::from("We Just Laugh About It"),
        src: String::from("https://www.ytmp3.cn/down/77699.mp3"),
        describe: String::from("2007年瑞典歌手Lucky Twice演唱的歌曲"),
    });
    Json(list)
}

#[get("/get_categories")]
fn get_categories() -> Json<Vec<Category>> {
    let categories = [
        "推荐", "华语", "日韩", "网络", "欧美", "现场", "热舞", "伤感", "剧情",
    ];

    let mut list: Vec<Category> = Vec::new();
    let mut i = 0;
    for name in categories.iter() {
        i += 1;
        list.push(Category {
            id: i,
            name: name.to_string(),
        });
    }
    Json(list)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![list, get_categories])
}
