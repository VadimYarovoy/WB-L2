// Принимаем срез целых чисел (&[i32]), возвращаем получателя (rx).
fn as_chan(vs: &[i32]) -> std::sync::mpsc::Receiver<i32> {
    // Создает канал для передачи данных
    let (tx, rx) = std::sync::mpsc::channel();

    // Запускает новый поток, который отправляет каждое значение из переданного среза в канал с задержкой в 1 секунду
    let handle = std::thread::spawn({
        let vs = vs.to_owned();

        move || {
            for v in vs {
                tx.send(v).unwrap();
                // Сама задержка
                std::thread::sleep(std::time::Duration::from_secs(1))
            }
            // После завершения отправки значений, отправитель закрывается с помощью drop(tx).
            drop(tx);
        }
    });

    // Ждем завершение работы попока
    handle.join().unwrap();

    rx
}

// Принимаем два получателя
fn merge(
    a: std::sync::mpsc::Receiver<i32>,
    b: std::sync::mpsc::Receiver<i32>,
) -> std::sync::mpsc::Receiver<i32> {
    // Создаем новый канал
    let (tx, rx) = std::sync::mpsc::channel();

    // Создаем флаги для проверки, что каналы не пустые
    let mut a_done = false;

    let mut b_done = false;

    // Бесконечный цикл
    loop {
        // Проверяем есть ли еще занчения в канале
        match a.try_recv() {
            Ok(i) => {
                // Если есть то отправляем в новый канал
                tx.send(i).unwrap();
            }

            Err(_) => {
                // Если нет то выставляем флаг того, что канал пустой
                a_done = true;
            }
        }

        // Аналогично первому
        match b.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap();
            }

            Err(_) => {
                b_done = true;
            }
        }

        // Если оба канала пусты - выходим из цикла
        if a_done && b_done {
            break;
        }
    }

    // возвращаем получателя со значениии идущими по очереди (в случае если кол-во сообщений в каналах равно)
    rx
}

fn main() {
    let a = as_chan(&vec![1, 3, 5, 7]);

    let b = as_chan(&vec![2, 4, 6, 8]);

    let c = merge(a, b);

    // Получаем сообщения из канала и печатаем их
    for v in c.iter() {
        println!("{v:?}");
    }
}
