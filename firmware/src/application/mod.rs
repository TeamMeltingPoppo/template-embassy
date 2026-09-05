/* 上位の層の処理はここに書く（キャリブレーション等）
 */

pub async fn task1() {
    loop {
        let instant = embassy_time::Instant::now();
        defmt::info!("It works! (t={}us)", &&instant.as_micros());
        embassy_time::Timer::after_millis(1000).await;
    }
}
