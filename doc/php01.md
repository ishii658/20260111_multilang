# php の環境

apacheではなく単独で利用できる環境　

# install

```
sudo apt update
sudo apt install php php-mbstring
sudo apt-get install php-dom php-curl
sudo apt install php-pear
sudo apt install php-dev
```

# 実行

```
php php01.php
```

# php02 webサーバー

```
# 質問の回答はnoで良い
sudo pecl install swoole
# PHPの設定ファイルにSwooleを追加
# php.iniに追加するか、新しいファイルを作成
sudo nano /etc/php/8.x/cli/conf.d/20-swoole.ini
# 内容は以下
extension=swoole


mkdir php02
composer create-project laravel/laravel .
composer require laravel/octane
php artisan octane:install --server=swoole

# サーバー start
php artisan octane:start --host=127.0.0.1 --port=8080
```

```
../static のファイルをpublicにコピー
```

## api 

* routes/api.php

```
use Illuminate\Support\Facades\Route;

Route::get('/test', function () {
    return response()->json(['message' => 'OK']);
});
```

http://127.0.0.1:8080/api/test で json を返す