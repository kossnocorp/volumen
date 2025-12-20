use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use volumen_parser_core::VolumenParser;

// Small code sample - basic prompt detection
const SMALL_GO: &str = r#"
package main

// @prompt
var userPrompt = "You are a helpful assistant."
var systemPrompt = `You are a system.`
"#;

// Medium code sample - multiple prompts with annotations
const MEDIUM_GO: &str = r#"
package main

import "fmt"

type ChatBot struct {
    systemPrompt  string
    userGreeting  string
    regularVar    int
}

func NewChatBot() *ChatBot {
    // @prompt
    systemPrompt := "You are a helpful AI assistant."

    // @prompt
    userGreeting := `Hello user!`

    return &ChatBot{
        systemPrompt: systemPrompt,
        userGreeting: userGreeting,
        regularVar:   42,
    }
}

func (cb *ChatBot) Process(message string) string {
    // @prompt
    contextPrompt := fmt.Sprintf(`
You are processing: %s
Please respond accordingly.
`, message)

    responseTemplate := fmt.Sprintf("Response to %s", message)

    return cb.Generate(contextPrompt)
}

func (cb *ChatBot) Generate(prompt string) string {
    // @prompt
    system := "Generate a response"
    // @prompt
    user := fmt.Sprintf("Based on: %s", prompt)

    return cb.model.Run(system, user)
}

// @prompt
var globalPrompt = "This is a global prompt"

func helper() string {
    localPrompt := "Local value"
    return localPrompt
}
"#;

// Large code sample - complex nested structures
const LARGE_GO: &str = r#"
package main

import (
    "fmt"
    "strings"
)

type AssistantConfig struct {
    Role         string
    Capabilities string
}

type AIAssistant struct {
    systemPrompt     string
    greetingTemplate string
    userPrefs        AssistantConfig
    model            *ModelClient
}

func NewAIAssistant(config AssistantConfig) *AIAssistant {
    // @prompt
    systemPrompt := fmt.Sprintf("You are %s with %s.", config.Role, config.Capabilities)

    // @prompt
    greetingTemplate := `
Hello! I'm your AI assistant.
I can help you with various tasks.
`

    return &AIAssistant{
        systemPrompt:     systemPrompt,
        greetingTemplate: greetingTemplate,
        userPrefs:        config,
        model:            NewModelClient(),
    }
}

func (a *AIAssistant) ProcessQuery(query string, context *string) (string, error) {
    // @prompt
    basePrompt := fmt.Sprintf("Process query: %s", query)

    var promptToUse string

    if context != nil {
        // @prompt
        contextualizedPrompt := fmt.Sprintf(`
Query: %s
Context: %s
Please provide a detailed response.
`, query, *context)
        promptToUse = contextualizedPrompt
    } else {
        promptToUse = basePrompt
    }

    return a.GenerateResponse(promptToUse)
}

func (a *AIAssistant) GenerateResponse(prompt string) (string, error) {
    // @prompt
    systemInstruction := "Generate a helpful and accurate response"
    // @prompt
    userMessage := fmt.Sprintf("Based on: %s", prompt)

    result, err := a.model.Complete(systemInstruction, userMessage)
    if err != nil {
        return "", err
    }
    return a.PostProcess(result), nil
}

func (a *AIAssistant) PostProcess(text string) string {
    // Non-prompt processing
    return strings.TrimSpace(text)
}

func CreateFromConfig(configPath string) (*AIAssistant, error) {
    config, err := loadConfig(configPath)
    if err != nil {
        return nil, err
    }
    return NewAIAssistant(config), nil
}

type ConversationManager struct {
    conversationStart string
    history           []string
}

func NewConversationManager() *ConversationManager {
    // @prompt
    conversationStart := "Let's begin our conversation."

    return &ConversationManager{
        conversationStart: conversationStart,
        history:           make([]string, 0),
    }
}

func (cm *ConversationManager) AddMessage(message string) {
    cm.history = append(cm.history, message)

    if len(cm.history) > 10 {
        // @prompt
        summaryPrompt := fmt.Sprintf(`
Summarize the following conversation:
%s
`, strings.Join(cm.history, "\n"))
        cm.Summarize(summaryPrompt)
    }
}

func (cm *ConversationManager) Summarize(prompt string) string {
    // @prompt
    system := "You are a conversation summarizer"
    // @prompt
    user := fmt.Sprintf("Summarize: %s", prompt)

    return generateSummary(system, user)
}

// @prompt
const DEFAULT_SYSTEM_PROMPT = "You are a helpful AI assistant"

// @prompt
var FALLBACK_PROMPT = fmt.Sprintf(`
If no other prompt is available,
use this fallback: %s
`, DEFAULT_SYSTEM_PROMPT)

func createSpecializedPrompt(task, domain string) string {
    // @prompt
    specialized := fmt.Sprintf(`
Task: %s
Domain: %s
Please provide expert-level assistance.
`, task, domain)
    return specialized
}

func main() {
    assistant := NewAIAssistant(AssistantConfig{
        Role:         "helper",
        Capabilities: "general",
    })

    response, err := assistant.ProcessQuery("What is Go?", nil)
    if err != nil {
        panic(err)
    }
    fmt.Println(response)
}
"#;

fn bench_go_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("go_parsers");

    // Small benchmark
    group.throughput(Throughput::Bytes(SMALL_GO.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "small"),
        &SMALL_GO,
        |b, code| {
            b.iter(|| volumen_parser_go::ParserGo::parse(black_box(code), "test.go"));
        },
    );

    // Medium benchmark
    group.throughput(Throughput::Bytes(MEDIUM_GO.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "medium"),
        &MEDIUM_GO,
        |b, code| {
            b.iter(|| volumen_parser_go::ParserGo::parse(black_box(code), "test.go"));
        },
    );

    // Large benchmark
    group.throughput(Throughput::Bytes(LARGE_GO.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "large"),
        &LARGE_GO,
        |b, code| {
            b.iter(|| volumen_parser_go::ParserGo::parse(black_box(code), "test.go"));
        },
    );

    group.finish();
}

criterion_group!(benches, bench_go_parsers);
criterion_main!(benches);
