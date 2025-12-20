use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use volumen_parser_core::VolumenParser;

// Small code sample - basic prompt detection
const SMALL_PHP: &str = r#"
<?php
// @prompt
$userPrompt = "You are a helpful assistant.";
$systemPrompt = "You are a {$role}.";
?>
"#;

// Medium code sample - multiple prompts with annotations
const MEDIUM_PHP: &str = r#"
<?php
class ChatBot {
    private $systemPrompt;
    private $userGreeting;
    private $regularVar;

    public function __construct() {
        // @prompt
        $this->systemPrompt = "You are a helpful AI assistant.";

        // @prompt
        $this->userGreeting = "Hello {$this->name}!";

        $this->regularVar = 42;
    }

    public function process($message) {
        // @prompt
        $contextPrompt = <<<HEREDOC
You are processing: {$message}
Please respond accordingly.
HEREDOC;

        $responseTemplate = "Response to {$message}";

        return $this->generate($contextPrompt);
    }

    public function generate($prompt) {
        // @prompt
        $system = "Generate a response";
        // @prompt
        $user = "Based on: {$prompt}";

        return $this->model->run($system, $user);
    }
}

// @prompt
$globalPrompt = "This is a global prompt";

function helper() {
    $localPrompt = "Local {$value}";
    return $localPrompt;
}
?>
"#;

// Large code sample - complex nested structures
const LARGE_PHP: &str = r#"
<?php
require_once 'types.php';

interface AssistantConfig {
    public function getRole(): string;
    public function getCapabilities(): string;
}

class AIAssistant {
    private $systemPrompt;
    private $greetingTemplate;
    private $userPrefs;
    private $model;

    public function __construct(AssistantConfig $config) {
        // @prompt
        $this->systemPrompt = "You are {$config->getRole()} with {$config->getCapabilities()}.";

        // @prompt
        $this->greetingTemplate = <<<HEREDOC
Hello! I'm your AI assistant.
I can help you with various tasks.
HEREDOC;

        $this->userPrefs = $config;
        $this->model = new ModelClient();
    }

    public function processQuery($query, $context = null) {
        // @prompt
        $basePrompt = "Process query: {$query}";

        if ($context !== null) {
            // @prompt
            $contextualizedPrompt = <<<HEREDOC
Query: {$query}
Context: {$context}
Please provide a detailed response.
HEREDOC;
            $promptToUse = $contextualizedPrompt;
        } else {
            $promptToUse = $basePrompt;
        }

        return $this->generateResponse($promptToUse);
    }

    public function generateResponse($prompt) {
        // @prompt
        $systemInstruction = "Generate a helpful and accurate response";
        // @prompt
        $userMessage = "Based on: {$prompt}";

        $result = $this->model->complete($systemInstruction, $userMessage);
        return $this->postProcess($result);
    }

    public function postProcess($text) {
        // Non-prompt processing
        return trim($text);
    }

    public static function createFromConfig($configPath) {
        $config = loadConfig($configPath);
        return new self($config);
    }
}

class ConversationManager {
    private $conversationStart;
    private $history;

    public function __construct() {
        // @prompt
        $this->conversationStart = "Let's begin our conversation.";
        $this->history = [];
    }

    public function addMessage($message) {
        $this->history[] = $message;

        if (count($this->history) > 10) {
            // @prompt
            $summaryPrompt = <<<HEREDOC
Summarize the following conversation:
{implode("\n", $this->history)}
HEREDOC;
            $this->summarize($summaryPrompt);
        }
    }

    public function summarize($prompt) {
        // @prompt
        $system = "You are a conversation summarizer";
        // @prompt
        $user = "Summarize: {$prompt}";

        return generateSummary($system, $user);
    }
}

// @prompt
const DEFAULT_SYSTEM_PROMPT = "You are a helpful AI assistant";

// @prompt
$FALLBACK_PROMPT = <<<HEREDOC
If no other prompt is available,
use this fallback: {DEFAULT_SYSTEM_PROMPT}
HEREDOC;

function createSpecializedPrompt($task, $domain) {
    // @prompt
    $specialized = <<<HEREDOC
Task: {$task}
Domain: {$domain}
Please provide expert-level assistance.
HEREDOC;
    return $specialized;
}

function main() {
    $assistant = new AIAssistant(new Config([
        'role' => 'helper',
        'capabilities' => 'general'
    ]));

    $response = $assistant->processQuery("What is PHP?");
    echo $response;
}
?>
"#;

fn bench_php_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("php_parsers");

    // Small benchmark
    group.throughput(Throughput::Bytes(SMALL_PHP.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "small"),
        &SMALL_PHP,
        |b, code| {
            b.iter(|| volumen_parser_php::ParserPhp::parse(black_box(code), "test.php"));
        },
    );

    // Medium benchmark
    group.throughput(Throughput::Bytes(MEDIUM_PHP.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "medium"),
        &MEDIUM_PHP,
        |b, code| {
            b.iter(|| volumen_parser_php::ParserPhp::parse(black_box(code), "test.php"));
        },
    );

    // Large benchmark
    group.throughput(Throughput::Bytes(LARGE_PHP.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "large"),
        &LARGE_PHP,
        |b, code| {
            b.iter(|| volumen_parser_php::ParserPhp::parse(black_box(code), "test.php"));
        },
    );

    group.finish();
}

criterion_group!(benches, bench_php_parsers);
criterion_main!(benches);
