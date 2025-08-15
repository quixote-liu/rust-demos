use actix_files::NamedFile;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use bytes::Bytes;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::PathBuf;
// use tokio::task::block_in_place;

const CHUNK_SIZE: usize = 4 * 1024 * 1024; // 4MB

#[get("/stream-file/{filename}")]
pub async fn stream_file(path: actix_web::web::Path<String>) -> impl Responder {
    let filename = path.into_inner();
    let file_path = PathBuf::from(filename);

    // 检查文件是否存在
    if !file_path.exists() {
        return HttpResponse::NotFound().body("File not found");
    }

    // 构建响应流
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(FileStream::with_file_path(file_path))
}

struct FileStream<R> {
    reader: R,
}

impl <R: Read> FileStream<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: reader,
        }
    }

    fn with_file_path(file_path: PathBuf) -> Self {
        let mut file = File::open(file_path).unwrap();
        Self {
            reader: file,
        }
    }
}

impl <R: Read + Unpin> futures::Stream for FileStream<R> {
    type Item = io::Result<Bytes>;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        // 读取4MB块
        let mut buffer = vec![0; CHUNK_SIZE];
        match self.reader.read(&mut buffer) {
            Ok(0) => std::task::Poll::Ready(None), // 读取完毕
            Ok(n) => {
                buffer.truncate(n);
                std::task::Poll::Ready(Some(Ok(Bytes::from(buffer))))
            }
            Err(e) => std::task::Poll::Ready(Some(Err(e))),
        }
    }
}

async fn foo() -> u8 { 
    88
}

#[get("/test")]
pub async fn anync_test() -> impl Responder {
    println!("开始执行1");
    async {
        let x: u8 = foo().await;
        println!("xx = {}", x);

        println!("开始读取文件");
        // 开始读取文件，进入IO阻塞
        let file = File::open("C:\\code\\rust-demos\\src\\file_demo\\br_compress\\AI_ans1");
        if let Ok(mut f) = file {
            let mut s = String::new();
            let _ = f.read_to_string(&mut s).unwrap();
            println!("{s}");
        }
        println!("读取文件完毕");
    }.await;
    println!("开始执行2");
    
    HttpResponse::Ok()
}
