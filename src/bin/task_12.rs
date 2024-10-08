fn main() {
    // Объявляем статический неизсеняемый массив "a" размером 5 с автоматически выведенным типом i32, который содержит пять целых чисел. В Rust массивы имеют фиксированную длину
    let  a = [76, 77, 78, 79, 80];

    // Массив занимает в памяти ровно i32 * 5 = 20 байт, никаких накладных расходов, все элементы должны быть одного типа
    // let arr_size = std::mem::size_of_val(&a);
    // println!("{:?}", arr_size);

    // Cоздаем срез "b", который ссылается на часть массива "a"
    // Срезы позволяют работать с подмножеством массива или вектора без копирования данных.
    // Берем элементы массива a с индексами от 1 до 3 (4 не включительно)
    let b = &a[1..4];

    // Мы по преджену можем обращаться к "a", так как у нас только неизменяемый доступ по ссылке, например
    // println!("{a:?}");

    // Есть еще важный момент, если бы "a" был бы изменяемым, то есть let mut a, то мы бы не могли изменять его пока существуест срез
    // a[0] = 0; - Вызывало бы конфликт правил заимствования (либо одна уникальная ссылка, либо много разделяемых ссылок)

    // :? используется для форматирования вывода в "отладочном" стиле
    println!("{b:?}");

    // Выводом будет
    // [77, 78, 79]
}
