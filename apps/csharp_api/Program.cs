var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

app.MapGet("/", () => Results.Json(new { message = "Hello from ASP.NET Core" }));

app.Run();
