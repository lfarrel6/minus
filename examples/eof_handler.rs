use minus::error::MinusError;
use tokio::{join, sync::mpsc::channel, task::spawn_blocking};

#[tokio::main]
async fn main() -> Result<(), MinusError> {
    let (sender, mut receiver) = channel(10);

    let output = minus::Pager::new();

    for i in 0..=50_u32 {
        output.push_str(&format!("{}\n", i))?;
    }

    output.add_eof_callback(Box::new(move |x: usize, y: usize| {
        let mut buf = Vec::with_capacity(10);
        for i in 0..=10_u32 {
            buf.push(format!("({x},{y}) {i}\n"));
        }
        let owned_sender = sender.clone();
        tokio::task::spawn(async move {
          owned_sender.send(buf).await;
        });
    }));

    let increment = async {
        while let Some(buf) = receiver.recv().await {
            for line in buf {
                output.push_str(&line)?;
            }
        }
        Result::<_, MinusError>::Ok(())
    };

    let output = output.clone();
    let (res1, res2) = join!(
        spawn_blocking(move || minus::dynamic_paging(output)),
        increment
    );
    res1.unwrap()?;
    res2?;
    Ok(())
}
