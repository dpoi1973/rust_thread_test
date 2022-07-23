use crate::services::http_service;

//参数生命周期
#[tokio::main]
pub async fn getAvatarArrayFromPlatformServer(
    url: &'static str,
    token: &'static str,
    array: Vec<&'static str>,
) -> Vec<String> {
    let mut array = array.chunks(1);
    // println!("in one : {:?}", array);
    let mut threads = vec![];

    for item in array {
        // println!("for:{:?}", item);
        let mut arr = item[0];
        let h = tokio::spawn(async move { http_service::get(url, token, &arr).await });
        threads.push(h);
    }
    let mut ids = vec![];

    for th in threads {
        // println!("okok{:?}", kk);
        let thres = th.await;
        match thres {
            Ok(result) => match result {
                Ok(res) => {
                    // println!("result:{:?}", res["data"]["list"][0]["_id"]);
                    ids.push(res["data"]["list"][0]["_id"].to_string());
                }
                Err(e) => panic!("{:?}", e),
            },
            Err(err) => panic!(),
        }
    }
    // tokio::try_join!().unwrap();
    // println!("this ids is:{:?}", ids);
    ids
}
