use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use volumen_parser_core::VolumenParser;

// Small code sample - basic prompt detection
const SMALL_TYPESCRIPT: &str = r#"
// @prompt
const userPrompt = "You are a helpful assistant.";
const systemPrompt = `You are a ${role}.`;
"#;

// Medium code sample - multiple prompts with annotations
const MEDIUM_TYPESCRIPT: &str = r#"
class ChatBot {
    private systemPrompt: string;
    private userGreeting: string;
    private regularVar: number;

    constructor() {
        // @prompt
        this.systemPrompt = "You are a helpful AI assistant.";

        // @prompt
        this.userGreeting = `Hello ${this.name}!`;

        this.regularVar = 42;
    }

    process(message: string): string {
        // @prompt
        const contextPrompt = `
            You are processing: ${message}
            Please respond accordingly.
        `;

        const responseTemplate = `Response to ${message}`;

        return this.generate(contextPrompt);
    }

    generate(prompt: string): string {
        // @prompt
        const system = "Generate a response";
        // @prompt
        const user = `Based on: ${prompt}`;

        return this.model.run(system, user);
    }
}

// @prompt
const globalPrompt = "This is a global prompt";

function helper(): string {
    const localPrompt = `Local ${value}`;
    return localPrompt;
}
"#;

// Large code sample - complex nested structures
const LARGE_TYPESCRIPT: &str = r#"
import { Configuration, ModelClient } from './types';

interface AssistantConfig {
    role: string;
    capabilities: string;
}

class AIAssistant {
    private systemPrompt: string;
    private greetingTemplate: string;
    private userPrefs: AssistantConfig;
    private model: ModelClient;

    constructor(config: AssistantConfig) {
        // @prompt
        this.systemPrompt = `You are ${config.role} with ${config.capabilities}.`;

        // @prompt
        this.greetingTemplate = `
            Hello! I'm your AI assistant.
            I can help you with various tasks.
        `;

        this.userPrefs = config;
    }

    async processQuery(query: string, context?: string): Promise<string> {
        // @prompt
        const basePrompt = `Process query: ${query}`;

        let promptToUse: string;

        if (context) {
            // @prompt
            const contextualizedPrompt = `
                Query: ${query}
                Context: ${context}
                Please provide a detailed response.
            `;
            promptToUse = contextualizedPrompt;
        } else {
            promptToUse = basePrompt;
        }

        return await this.generateResponse(promptToUse);
    }

    async generateResponse(prompt: string): Promise<string> {
        // @prompt
        const systemInstruction = "Generate a helpful and accurate response";
        // @prompt
        const userMessage = `Based on: ${prompt}`;

        const result = await this.model.complete(systemInstruction, userMessage);
        return this.postProcess(result);
    }

    postProcess(text: string): string {
        // Non-prompt processing
        const cleaned = text.trim();
        return cleaned;
    }

    static createFromConfig(configPath: string): AIAssistant {
        const config = loadConfig(configPath);
        return new AIAssistant(config);
    }
}

class ConversationManager {
    private conversationStart: string;
    private history: string[];

    constructor() {
        // @prompt
        this.conversationStart = "Let's begin our conversation.";
        this.history = [];
    }

    addMessage(message: string): void {
        this.history.push(message);

        if (this.history.length > 10) {
            // @prompt
            const summaryPrompt = `
                Summarize the following conversation:
                ${this.history.join('\n')}
            `;
            this.summarize(summaryPrompt);
        }
    }

    summarize(prompt: string): string {
        // @prompt
        const system = "You are a conversation summarizer";
        // @prompt
        const user = `Summarize: ${prompt}`;

        return generateSummary(system, user);
    }
}

// @prompt
const DEFAULT_SYSTEM_PROMPT = "You are a helpful AI assistant";

// @prompt
const FALLBACK_PROMPT = `
    If no other prompt is available,
    use this fallback: ${DEFAULT_SYSTEM_PROMPT}
`;

function createSpecializedPrompt(task: string, domain: string): string {
    // @prompt
    const specialized = `
        Task: ${task}
        Domain: ${domain}
        Please provide expert-level assistance.
    `;
    return specialized;
}

async function main(): Promise<void> {
    const assistant = new AIAssistant({
        role: 'helper',
        capabilities: 'general'
    });

    const response = await assistant.processQuery("What is TypeScript?");
    console.log(response);
}

export { AIAssistant, ConversationManager, createSpecializedPrompt };
"#;

fn bench_typescript_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("typescript_parsers");

    // Small benchmark
    group.throughput(Throughput::Bytes(SMALL_TYPESCRIPT.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Oxc", "small"),
        &SMALL_TYPESCRIPT,
        |b, code| {
            b.iter(|| volumen_parser_ts::ParserTs::parse(black_box(code), "test.ts"));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "small"),
        &SMALL_TYPESCRIPT,
        |b, code| {
            b.iter(|| volumen_parser_ts_tree_sitter::ParserTs::parse(black_box(code), "test.ts"));
        },
    );

    // Medium benchmark
    group.throughput(Throughput::Bytes(MEDIUM_TYPESCRIPT.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Oxc", "medium"),
        &MEDIUM_TYPESCRIPT,
        |b, code| {
            b.iter(|| volumen_parser_ts::ParserTs::parse(black_box(code), "test.ts"));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "medium"),
        &MEDIUM_TYPESCRIPT,
        |b, code| {
            b.iter(|| volumen_parser_ts_tree_sitter::ParserTs::parse(black_box(code), "test.ts"));
        },
    );

    // Large benchmark
    group.throughput(Throughput::Bytes(LARGE_TYPESCRIPT.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Oxc", "large"),
        &LARGE_TYPESCRIPT,
        |b, code| {
            b.iter(|| volumen_parser_ts::ParserTs::parse(black_box(code), "test.ts"));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "large"),
        &LARGE_TYPESCRIPT,
        |b, code| {
            b.iter(|| volumen_parser_ts_tree_sitter::ParserTs::parse(black_box(code), "test.ts"));
        },
    );

    group.finish();
}

criterion_group!(benches, bench_typescript_parsers);
criterion_main!(benches);
