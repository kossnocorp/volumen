use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use volumen_parser_core::VolumenParser;

// Small code sample - basic prompt detection
const SMALL_CSHARP: &str = r#"
public class Program
{
    // @prompt
    private static string userPrompt = "You are a helpful assistant.";
    private static string systemPrompt = $"You are a {role}.";
}
"#;

// Medium code sample - multiple prompts with annotations
const MEDIUM_CSHARP: &str = r#"
public class ChatBot
{
    private string systemPrompt;
    private string userGreeting;
    private int regularVar;

    public ChatBot()
    {
        // @prompt
        systemPrompt = "You are a helpful AI assistant.";

        // @prompt
        userGreeting = $"Hello {name}!";

        regularVar = 42;
    }

    public string Process(string message)
    {
        // @prompt
        string contextPrompt = $@"
You are processing: {message}
Please respond accordingly.
";

        string responseTemplate = $"Response to {message}";

        return Generate(contextPrompt);
    }

    public string Generate(string prompt)
    {
        // @prompt
        string system = "Generate a response";
        // @prompt
        string user = $"Based on: {prompt}";

        return model.Run(system, user);
    }
}

// @prompt
string globalPrompt = "This is a global prompt";

public string Helper()
{
    string localPrompt = $"Local {value}";
    return localPrompt;
}
"#;

// Large code sample - complex nested structures
const LARGE_CSHARP: &str = r#"
using System;
using System.Collections.Generic;
using System.Linq;

public interface IAssistantConfig
{
    string Role { get; }
    string Capabilities { get; }
}

public class AIAssistant
{
    private string systemPrompt;
    private string greetingTemplate;
    private IAssistantConfig userPrefs;
    private ModelClient model;

    public AIAssistant(IAssistantConfig config)
    {
        // @prompt
        systemPrompt = $"You are {config.Role} with {config.Capabilities}.";

        // @prompt
        greetingTemplate = @"
Hello! I'm your AI assistant.
I can help you with various tasks.
";

        userPrefs = config;
        model = new ModelClient();
    }

    public string ProcessQuery(string query, string context = null)
    {
        // @prompt
        string basePrompt = $"Process query: {query}";

        string promptToUse;

        if (context != null)
        {
            // @prompt
            string contextualizedPrompt = $@"
Query: {query}
Context: {context}
Please provide a detailed response.
";
            promptToUse = contextualizedPrompt;
        }
        else
        {
            promptToUse = basePrompt;
        }

        return GenerateResponse(promptToUse);
    }

    public string GenerateResponse(string prompt)
    {
        // @prompt
        string systemInstruction = "Generate a helpful and accurate response";
        // @prompt
        string userMessage = $"Based on: {prompt}";

        string result = model.Complete(systemInstruction, userMessage);
        return PostProcess(result);
    }

    public string PostProcess(string text)
    {
        // Non-prompt processing
        return text.Trim();
    }

    public static AIAssistant CreateFromConfig(string configPath)
    {
        var config = LoadConfig(configPath);
        return new AIAssistant(config);
    }
}

public class ConversationManager
{
    private string conversationStart;
    private List<string> history;

    public ConversationManager()
    {
        // @prompt
        conversationStart = "Let's begin our conversation.";
        history = new List<string>();
    }

    public void AddMessage(string message)
    {
        history.Add(message);

        if (history.Count > 10)
        {
            // @prompt
            string summaryPrompt = $@"
Summarize the following conversation:
{string.Join("\n", history)}
";
            Summarize(summaryPrompt);
        }
    }

    public string Summarize(string prompt)
    {
        // @prompt
        string system = "You are a conversation summarizer";
        // @prompt
        string user = $"Summarize: {prompt}";

        return GenerateSummary(system, user);
    }
}

// @prompt
const string DEFAULT_SYSTEM_PROMPT = "You are a helpful AI assistant";

// @prompt
static readonly string FALLBACK_PROMPT = $@"
If no other prompt is available,
use this fallback: {DEFAULT_SYSTEM_PROMPT}
";

public string CreateSpecializedPrompt(string task, string domain)
{
    // @prompt
    string specialized = $@"
Task: {task}
Domain: {domain}
Please provide expert-level assistance.
";
    return specialized;
}

public class Program
{
    public static void Main(string[] args)
    {
        var assistant = new AIAssistant(new Config
        {
            Role = "helper",
            Capabilities = "general"
        });

        string response = assistant.ProcessQuery("What is C#?");
        Console.WriteLine(response);
    }
}
"#;

fn bench_csharp_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("csharp_parsers");

    // Small benchmark
    group.throughput(Throughput::Bytes(SMALL_CSHARP.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "small"),
        &SMALL_CSHARP,
        |b, code| {
            b.iter(|| volumen_parser_csharp::ParserCSharp::parse(black_box(code), "test.cs"));
        },
    );

    // Medium benchmark
    group.throughput(Throughput::Bytes(MEDIUM_CSHARP.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "medium"),
        &MEDIUM_CSHARP,
        |b, code| {
            b.iter(|| volumen_parser_csharp::ParserCSharp::parse(black_box(code), "test.cs"));
        },
    );

    // Large benchmark
    group.throughput(Throughput::Bytes(LARGE_CSHARP.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "large"),
        &LARGE_CSHARP,
        |b, code| {
            b.iter(|| volumen_parser_csharp::ParserCSharp::parse(black_box(code), "test.cs"));
        },
    );

    group.finish();
}

criterion_group!(benches, bench_csharp_parsers);
criterion_main!(benches);
