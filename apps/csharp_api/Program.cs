var builder = WebApplication.CreateBuilder(args);
builder.Services.AddResponseCompression();
var app = builder.Build();
app.UseResponseCompression();

app.MapGet("/", () => Results.Json(new { message = "Hello from ASP.NET Core" }));
app.MapGet("/health", () => Results.Ok("ok"));

var port = Environment.GetEnvironmentVariable("PORT") ?? "5000";
app.Urls.Add($"http://0.0.0.0:{port}");
app.Run();
