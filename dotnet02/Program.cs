using Microsoft.Extensions.FileProviders;

var builder = WebApplication.CreateBuilder(args);

// サービスの追加
builder.Services.AddOpenApi();

var app = builder.Build();

// 開発環境用の設定
if (app.Environment.IsDevelopment())
{
    app.MapOpenApi();
}

app.UseHttpsRedirection();

// ---------------------------------------------------------
// ここから: 静的ファイル配信の設定
// ---------------------------------------------------------

// プロジェクトのルート(ContentRootPath)から一つ上の階層にある "static" フォルダの絶対パスを取得
var staticPath = Path.GetFullPath(Path.Combine(builder.Environment.ContentRootPath, "..", "static"));

// フォルダが存在するか確認 (エラー回避のため)
if (Directory.Exists(staticPath))
{
    var fileProvider = new PhysicalFileProvider(staticPath);

    // 1. デフォルトファイル (index.html 等) をルート "/" で表示するための設定
    app.UseDefaultFiles(new DefaultFilesOptions
    {
        FileProvider = fileProvider,
        RequestPath = "" // ルート URL にマッピング
    });

    // 2. 静的ファイルを配信するための設定
    app.UseStaticFiles(new StaticFileOptions
    {
        FileProvider = fileProvider,
        RequestPath = "" // ルート URL にマッピング
    });
}
else
{
    // フォルダがない場合の警告ログ（必要に応じて）
    Console.WriteLine($"警告: 静的ファイルフォルダが見つかりません: {staticPath}");
}
// ---------------------------------------------------------
// ここまで
// ---------------------------------------------------------


var summaries = new[]
{
    "Freezing", "Bracing", "Chilly", "Cool", "Mild", "Warm", "Balmy", "Hot", "Sweltering", "Scorching"
};

// 既存の API エンドポイント
app.MapGet("/weatherforecast", () =>
{
    var forecast =  Enumerable.Range(1, 5).Select(index =>
        new WeatherForecast
        (
            DateOnly.FromDateTime(DateTime.Now.AddDays(index)),
            Random.Shared.Next(-20, 55),
            summaries[Random.Shared.Next(summaries.Length)]
        ))
        .ToArray();
    return forecast;
})
.WithName("GetWeatherForecast");

app.Run();

record WeatherForecast(DateOnly Date, int TemperatureC, string? Summary)
{
    public int TemperatureF => 32 + (int)(TemperatureC / 0.5556);
}