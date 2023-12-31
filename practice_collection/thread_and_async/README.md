# スレッドと非同期実行

## スレッドの利用方法

OSのスレッド機能を使い方法と、グリーンスレッドを利用する２種類がある。
 グリーンスレッドとは、ラインタイムライブラリや仮想マシンにより制御されるスレッドになる。
spawn()関数やメソッドはOSのスレッドを利用し、実行速度が大変早いと言うメリットがあるが、
その代わりに適切な制御が求められると同時に、Rustのライフタイムの問題を考慮した実装をする必要がある。
グリーンスレッドを利用して関数やメソッドを実行させるには外部クレートを使用する

- crossbeam crate

## Arc(Atomically Reference Counted)

スレッドセーフな参照カウントを実装したスマートポインタ
Arc::new() -> Arc::clone -> arc.wait() -> std::sync::BarrierWaitResult
BarrierWaitResult::is_reader() 最初に起動したスレッド（リーダースレッド）はtrue それ以外はfalse

## Box<dyn Any + Send>

Anyトレイトはあらゆる型の参照を扱うことが出来る。
Sendトレイトはスレッド間の所有権の転送を許可するマーカートレイト

## ScopedJoinHandle<'_, T>

ScopedJoinHandle<'_, u64> は、Rustの crossbeam クレートが提供する型であり、スコープ内で生成されたスレッドのハンドルを表します。crossbeam クレートは、スレッドセーフな並行処理をサポートするためのユーティリティを提供するためのものです。

ScopedJoinHandle は、crossbeam の scope 関数を使用して生成されたスレッドハンドルで、そのスレッドがスコープを抜けるときに自動的に終了します。これにより、スレッドのライフタイム管理をより簡潔に行うことができます。

具体的に、ScopedJoinHandle<'_, u64> の部分を分解して説明します：

ScopedJoinHandle: スレッドハンドルの型を表します。スコープ内で生成されるスレッドのライフタイムを表すことから、スコープ内でのみ有効なハンドルとなります。
<'_>: ライフタイムの省略形で、スコープ内のライフタイムを示します。スレッドの終了と共にハンドルが自動的に解放されるため、ライフタイムの指定が必要です。
u64: スレッドが返す値の型を指定します。スレッドが終了する際にその値が取得できます。

## スレッド間通信

標準ライブラリ
1. `std::sync::mpsc::channel::<T>()` 送受信のチャネルを作る(2つ)

   * -> `std::sync::mpsc::{Sender, Receiver}` チャネルがタプルで返される

2. `Sender::send() Receiver::recv()` 送信、待ち受け
3. `std::thread::spam` 送受信スレッド２つ実行
4. `JoinHandler::join()` スレッドの終了を待つ

crossbeamライブラリ
1. `crossbeam::channel::bounded<T>()` 送受信のチャネルを作る(2つ)

   * -> `crossbeam::channel::{Sender, Receiver}` チャネルがタプルで返される

2. `Sender::send() Receiver::recv()` 送信、待ち受け
3. `crossbeam::thread::scope` スコープ生成
4. `crossbeam::thread::scope::spawn` スレッドを２つ作る
5. `ScopedJoinHandle::join()` スレッドの終了を待つ

## 標準ライブラリの排他制御

> [!WARNING]
> `get` only gives you a reference to the items in a `Vec`, so you can't call methods that require ownership like `join`.
>
> Since there's only two handles, you can just assign them to their own variables.
> [QA](https://users.rust-lang.org/t/value-moved-std-thread-join/99014?u=friendbear)

### RwLock

> mpsc::channel::<()>(); send()を使って recv()待ちしているスレッドを開始させる
> JoinHandler::join()で結果を取得する。
> RwLockはwrite()で書き込みread()で読み込み

### crossbeamクレートの排他制御

> crossbeam::channel::<()>(); send()を使ってrecv()待ちしているスレッドを開始させる
> ShradeLock::{ShradeLockReadGuard<T>, ShradeLockWriteGuard<T>} のwrite()で書き込みread()で読み込み
> ScopedJoinHandler::join().unwarp().unwarp()で結果を取得