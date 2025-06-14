using Microsoft.AspNetCore.Diagnostics.HealthChecks;
var builder = WebApplication.CreateBuilder(args);
builder.Services.AddHealthChecks();
var app = builder.Build();

app.Lifetime.ApplicationStopping.Register(() =>
{
    app.Logger.LogInformation("Shutting down");
});

app.MapGet("/", () => Results.Json(new { message = "Hello from ASP.NET Core" }));
app.MapHealthChecks("/healthz");

var port = Environment.GetEnvironmentVariable("PORT") ?? "5000";
app.Urls.Add($"http://0.0.0.0:{port}");
app.Run();
