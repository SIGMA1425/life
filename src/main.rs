struct Sensor{
    active: bool,
    latest: u32,
}

impl Sensor{
    //メソッド，第一引数は必ず&self
    fn read(&self) -> u32{
        self.latest
    }

    //オブジェクトの値を変更するときは&mut
    fn init(&mut self){
        self.active = true;
        self.latest = 42;
    }

    //関連関数，selfを受け取らない．基本的にはnew()としてコンストラクタとする
    fn new() -> Sensor{
        Sensor{
            active: false,
            latest: 0,
        }
    }
}


/* 'aはライフタイムパラメータ */
/* ライフタイムの関係をRustコンパイラに指示する */
/* 普段はライフタイムパラメータはコンパイラが推論してくれるが構造体や関数など */
/* 境界をまたいで参照を利用する際は明示する */
struct Image<'a>{
    raw: &'a [u8; 256],
}

fn main() {
    /* 所有権の基本（スコープ）*/
    {
        //ブロックスコープ内だと使える
        let x = 42;
        println!("{}", x);
    }

    //スコープ外だから使えない
    //println!("{}", x);


    /* 所有権のコピー */
    //i32のような型だと値のコピーが作られ，x, yはそれぞれ所有権を持つ
    let x = 42;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);

    
    /* 所有権のムーブ */
    //構造体のような型だと所有権はムーブされるためこの例だとsのメモリ領域は未初期化の状態になり，アクセスできなくなる
    let mut s = Sensor::new();
    s.init();
    let t = s;
    //println!("s.latest = {}", s.latest);
    println!("t.latest = {}", t.latest);

    //所有権がコピーされるかどうかはCopyトレイトが実装されているかで決まる


    /* 所有権の借用 */
    let sensor = Sensor::new();
    use_sensor(sensor);
    //所有権はsにムーブしたままなのでsensorにはアクセスできなくなる
    //println!("sensor.latest = {}", sensor.latest);

    let sb = Sensor::new();
    use_sensor_borrow(&sb);
    println!("sb.latest = {}", sb.latest);

    /* ライフタイム */
    /* 所有権を貸し出した場合貸した参照よりも長く生存しなければ所有者がいなくなった後で不正な参照をされる */
    let rx;
    {
        //xのスコープ開始
        let x = 42;
        //rxのライフタイムはじまり
        rx = &x;
        //xのスコープ終了
    }
    //この時点で参照元のxはスコープを抜けているため破棄されているため，このprintマクロはコンパイルエラーとなる
    //println!("rx = {}", rx);
    //rxのライフタイムおわり
    /* 値の所有者のスコープは参照する変数のライフタイムを包含している必要がある */


    /* 参照を含む構造体 */
    /* 画像データのような大きなデータはムーブもコピーも避けて参照でアクセスしなければ */
    /* メモリ内にデータの複製が作られオーバーヘッドが大きくなる */
    let image;
    {
        let bytes = [0; 256];
        image = Image{
            raw: &bytes,
        };
    }//bytesのスコープはここまで
    //println!("the first byte in Image = {}", image.raw[0]);
    //bytesのスコープとimageのライフタイムがずれている

}

fn use_sensor(s: Sensor){
    // sensorを使うとする
    println!("s.latest = {}", s.latest);
    //main内のsensorの所有権がsにムーブされ，sのスコープはここで抜ける

    //ムーブだけで回避するためには明示的に所有権を返す必要がある
}

//引数に参照を表す&を用いることで所有権をムーブすることなく値を貸し出す
//値を変更できる参照，可変参照を作る時は&mutをつける．そのとき，貸し出す値も可変である必要がある
fn use_sensor_borrow(sb: &Sensor){
    //値の所有者はあくまでもmain内の変数sensor
    println!("sb.latest = {}", sb.latest);
}
