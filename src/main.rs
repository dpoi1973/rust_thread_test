mod services;

fn main() {
    println!("Hello, world!");
    let url = "https://api-dev.tooqing.com/account/list";
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjVjYWFmZDgxZWI4MmE4NmY2YTM0YTJlMiIsInJvbGUiOjAsImNyZWRlbnRpYWwiOiJjdXJsLzcuNzkuMSIsImlhdCI6MTY1ODM3MTQ2MiwiZXhwIjoxNjYwOTk4MDIyfQ.mAOmETKFsnTvCVUBuzEWZZN-udt7vkqcYa_9SzHQfiw";
    //通过传入的数组执行分批多线程异步请求
    let result = services::api_service::getAvatarArrayFromPlatformServer(
        url,
        token,
        vec!["1", "2", "3", "4", "5"],
    );

    println!("result ids is: {:?}", result);
}
