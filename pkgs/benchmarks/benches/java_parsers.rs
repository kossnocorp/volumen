use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use volumen_parser_core::VolumenParser;

// Small code sample - basic prompt detection
const SMALL_JAVA: &str = r#"
public class Main {
    // @prompt
    private static String userPrompt = "You are a helpful assistant.";
    private static String systemPrompt = "You are a system.";
}
"#;

// Medium code sample - multiple prompts with annotations
const MEDIUM_JAVA: &str = r#"
public class ChatBot {
    private String systemPrompt;
    private String userGreeting;
    private int regularVar;

    public ChatBot() {
        // @prompt
        this.systemPrompt = "You are a helpful AI assistant.";

        // @prompt
        this.userGreeting = "Hello user!";

        this.regularVar = 42;
    }

    public String process(String message) {
        // @prompt
        String contextPrompt = """
You are processing: %s
Please respond accordingly.
""".formatted(message);

        String responseTemplate = String.format("Response to %s", message);

        return generate(contextPrompt);
    }

    public String generate(String prompt) {
        // @prompt
        String system = "Generate a response";
        // @prompt
        String user = String.format("Based on: %s", prompt);

        return model.run(system, user);
    }
}

// @prompt
String globalPrompt = "This is a global prompt";

public String helper() {
    String localPrompt = "Local value";
    return localPrompt;
}
"#;

// Large code sample - complex nested structures
const LARGE_JAVA: &str = r#"
import java.util.ArrayList;
import java.util.List;

interface AssistantConfig {
    String getRole();
    String getCapabilities();
}

class AIAssistant {
    private String systemPrompt;
    private String greetingTemplate;
    private AssistantConfig userPrefs;
    private ModelClient model;

    public AIAssistant(AssistantConfig config) {
        // @prompt
        this.systemPrompt = String.format("You are %s with %s.",
            config.getRole(), config.getCapabilities());

        // @prompt
        this.greetingTemplate = """
Hello! I'm your AI assistant.
I can help you with various tasks.
""";

        this.userPrefs = config;
        this.model = new ModelClient();
    }

    public String processQuery(String query, String context) {
        // @prompt
        String basePrompt = String.format("Process query: %s", query);

        String promptToUse;

        if (context != null) {
            // @prompt
            String contextualizedPrompt = """
Query: %s
Context: %s
Please provide a detailed response.
""".formatted(query, context);
            promptToUse = contextualizedPrompt;
        } else {
            promptToUse = basePrompt;
        }

        return generateResponse(promptToUse);
    }

    public String generateResponse(String prompt) {
        // @prompt
        String systemInstruction = "Generate a helpful and accurate response";
        // @prompt
        String userMessage = String.format("Based on: %s", prompt);

        String result = model.complete(systemInstruction, userMessage);
        return postProcess(result);
    }

    public String postProcess(String text) {
        // Non-prompt processing
        return text.trim();
    }

    public static AIAssistant createFromConfig(String configPath) {
        AssistantConfig config = loadConfig(configPath);
        return new AIAssistant(config);
    }
}

class ConversationManager {
    private String conversationStart;
    private List<String> history;

    public ConversationManager() {
        // @prompt
        this.conversationStart = "Let's begin our conversation.";
        this.history = new ArrayList<>();
    }

    public void addMessage(String message) {
        history.add(message);

        if (history.size() > 10) {
            // @prompt
            String summaryPrompt = """
Summarize the following conversation:
%s
""".formatted(String.join("\n", history));
            summarize(summaryPrompt);
        }
    }

    public String summarize(String prompt) {
        // @prompt
        String system = "You are a conversation summarizer";
        // @prompt
        String user = String.format("Summarize: %s", prompt);

        return generateSummary(system, user);
    }
}

// @prompt
static final String DEFAULT_SYSTEM_PROMPT = "You are a helpful AI assistant";

// @prompt
static final String FALLBACK_PROMPT = String.format("""
If no other prompt is available,
use this fallback: %s
""", DEFAULT_SYSTEM_PROMPT);

public String createSpecializedPrompt(String task, String domain) {
    // @prompt
    String specialized = """
Task: %s
Domain: %s
Please provide expert-level assistance.
""".formatted(task, domain);
    return specialized;
}

public class Main {
    public static void main(String[] args) {
        AIAssistant assistant = new AIAssistant(new Config(
            "helper",
            "general"
        ));

        String response = assistant.processQuery("What is Java?", null);
        System.out.println(response);
    }
}
"#;

fn bench_java_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("java_parsers");

    // Small benchmark
    group.throughput(Throughput::Bytes(SMALL_JAVA.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "small"),
        &SMALL_JAVA,
        |b, code| {
            b.iter(|| volumen_parser_java::ParserJava::parse(black_box(code), "test.java"));
        },
    );

    // Medium benchmark
    group.throughput(Throughput::Bytes(MEDIUM_JAVA.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "medium"),
        &MEDIUM_JAVA,
        |b, code| {
            b.iter(|| volumen_parser_java::ParserJava::parse(black_box(code), "test.java"));
        },
    );

    // Large benchmark
    group.throughput(Throughput::Bytes(LARGE_JAVA.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "large"),
        &LARGE_JAVA,
        |b, code| {
            b.iter(|| volumen_parser_java::ParserJava::parse(black_box(code), "test.java"));
        },
    );

    group.finish();
}

criterion_group!(benches, bench_java_parsers);
criterion_main!(benches);
