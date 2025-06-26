// use actix_web::{get, App, Error, HttpResponse, HttpServer};
// use futures::Stream;
// use std::{
//     path::Path,
//     pin::Pin,
//     task::{Context, Poll},
// };
// use tokio::{
//     fs::File,
//     io::{AsyncRead, ReadBuf},
// };

// // 自定义流结构体
// struct FileChunkStream {
//     file: File,
//     chunk_size: usize,
//     buffer: Vec<u8>,
//     finished: bool,
// }

// impl FileChunkStream {
//     fn new(file: File, chunk_size: usize) -> Self {
//         Self {
//             file,
//             chunk_size,
//             buffer: vec![0; chunk_size],
//             finished: false,
//         }
//     }
// }

// impl Stream for FileChunkStream {
//     type Item = Result<bytes::Bytes, std::io::Error>;

//     fn poll_next(
//         mut self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//     ) -> Poll<Option<Self::Item>> {
//         if self.finished {
//             return Poll::Ready(None);
//         }

//         let mut buffer = ReadBuf::new(&mut self.buffer);
//         let file = Pin::new(&mut self.file);

//         // 异步读取文件
//         match file.poll_read(cx, &mut buffer) {
//             Poll::Ready(Ok(())) => {
//                 let bytes_read = buffer.filled().len();
                
//                 if bytes_read == 0 {
//                     self.finished = true;
//                     Poll::Ready(None) // EOF
//                 } else {
//                     // 复制读取的数据块
//                     let chunk = bytes::Bytes::copy_from_slice(
//                         &self.buffer[..bytes_read]
//                     );
//                     Poll::Ready(Some(Ok(chunk)))
//                 }
//             }
//             Poll::Ready(Err(e)) => {
//                 self.finished = true;
//                 Poll::Ready(Some(Err(e)))
//             }
//             Poll::Pending => Poll::Pending,
//         }
//     }
// }

// #[get("/large-file")]
// async fn stream_large_file() -> Result<HttpResponse, Error> {
//     const CHUNK_SIZE: usize = 4 * 1024 * 1024; // 4MB
    
//     let path = Path::new("./large_file.dat");
    
//     // 打开文件
//     let file = match File::open(path).await {
//         Ok(file) => file,
//         Err(e) => {
//             return Err(actix_web::error::ErrorInternalServerError(e));
//         }
//     };
    
//     // 创建自定义流
//     let stream = FileChunkStream::new(file, CHUNK_SIZE);
    
//     // 设置内容类型和文件名
//     let content_disposition = format!(
//         "attachment; filename=\"{}\"",
//         path.file_name().unwrap().to_string_lossy()
//     );
    
//     Ok(HttpResponse::Ok()
//         .content_type("application/octet-stream")
//         .append_header(("Content-Disposition", content_disposition))
//         .streaming(stream))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(stream_large_file))
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }