struct Example(i32);

impl Drop for Example {
    // Переопределяем поведение Drop для Example.
    // Выводит значение e.0 на экран перед уничтожением.
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

struct ExampleWrap(Example);

impl Drop for ExampleWrap {
    // Переопределяем поведение Drop для ExampleWrap.
    // Когда экземпляр ExampleWrap уничтожается, он заменяет содержимое своего поля 0 на новый экземпляр Example(0) и сохраняет старое значение в переменной e.
    // Затем он выводит значение e.0 на экран. Это позволяет нам увидеть значение Example, которое было внутри ExampleWrap, перед его уничтожением.
    fn drop(&mut self) {
        let e = std::mem::replace(&mut self.0, Example(0));
        println!("wrap {}", e.0);
    }
}

fn main() {
    // создается временный экземпляр Example(1), который будет уничтожен сразу после этой строки, и вызовет drop, выводя 1.
    Example(1);

    // Создается экземпляр Example(2) и сохраняется в переменной _e2. Он будет уничтожен, когда выйдет из области видимости, что произойдет в конце функции main, и вызовет drop, выводя 2.
    // Если бы не было кастомного дропа, то анализатор бы увидел, что значение дальше не используется и дропнул бы его тут, а не в конце области.
    let _e2 = Example(2);

    // Тоже что и Example(2), но 2 напечатется первее так как стек разбирается в обратном направлении
    let _e3 = Example(3);

    // будет уничтожен сразу после этой строки, и вызовет drop, выводя 4. Та же логика что и в 1
    let _ = Example(4);

    // Здесь создается переменная _e5, которая будет использоваться для хранения значения типа Option<Example>.
    // Сначала в нее помещается Some(Example(5)), а затем она заменяется на None.
    // Это приводит к уничтожению экземпляра Example(5), и вызовет drop, выводя 5.
    let mut _e5;

    _e5 = Some(Example(5));

    _e5 = None;

    // Создается экземпляр Example(6), который затем явно уничтожается с помощью функции drop.
    // Это вызовет drop, выводя 6.
    let e6 = Example(6);

    drop(e6);

    // Cоздается экземпляр Example(7), но затем вызывается std::mem::forget(e7), что предотвращает вызов drop для этого экземпляра.
    let e7 = Example(7);

    // Мы фактически "забываем" о значении, и Rust не вызовает деструктор для этого значения.
    // Это может привести к утечкам памяти, если не освободить память вручную позже. (Что тут собствено и происходит)
    std::mem::forget(e7);

    // Создается вложенная структура
    // Первым вызовется дроп для ExampleWrap
    // Затем для Example(8), где он заменится на Example(0)
    // Последним для Example(0)
    ExampleWrap(Example(8));

    // Вывод программы
    // 1
    // 4
    // 5
    // 6
    // wrap 8
    // 8
    // 0
    // 3
    // 2
}
