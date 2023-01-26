use std::io::Cursor;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use reqwest;
use rodio::{Decoder, OutputStream, Sink};
use tempfile::Builder;
use error_chain::error_chain;
use tempfile::TempDir;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

async fn download_audio(tmp_dir: &mut TempDir, target: &str) -> Result<()> {
    let response = reqwest::get(target).await?;
    
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    let mut contents = Cursor::new(response.bytes().await?);
    copy(&mut contents, &mut dest)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://file-examples.com/storage/fe42c8472a63a1f029d7e90/2017/11/file_example_MP3_700KB.mp3";

    download_audio(&mut tmp_dir, target).await?;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(tmp_dir.path().join("file_example_MP3_700KB.mp3")).unwrap());

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
    Ok(())
}