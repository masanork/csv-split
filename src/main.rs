use csv::{ReaderBuilder, WriterBuilder};
use std::env;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // コマンドライン引数からファイル名を取得
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv_file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // CSVファイルを開く
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new().flexible(true).from_reader(file);

    let mut writers: Vec<csv::Writer<File>> = Vec::new();
    let mut first_row = true;

    for result in rdr.records() {
        let record = result?;

        if first_row {
            // 最初の行から属性の数を取得し、ファイルを準備する
            for i in 1..record.len() {
                let output_filename = format!("{}_{}.csv", filename.trim_end_matches(".csv"), i);
                let output_file = File::create(&output_filename)?;
                writers.push(WriterBuilder::new().flexible(true).from_writer(output_file));
            }
            first_row = false;
        }

        for (i, wtr) in writers.iter_mut().enumerate() {
            wtr.write_record(&[&record[0], &record[i+1]])?;
        }
    }

    // ファイルを正しく閉じる
    for mut wtr in writers {
        wtr.flush()?;
    }

    Ok(())
}
