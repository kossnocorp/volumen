use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

// Small code sample - basic prompt detection
const SMALL_PYTHON: &str = r#"
# @prompt
user_prompt = "You are a helpful assistant."
system_prompt = f"You are a {role}."
"#;

// Medium code sample - multiple prompts with annotations
const MEDIUM_PYTHON: &str = r#"
class ChatBot:
    def __init__(self):
        # @prompt
        self.system_prompt = "You are a helpful AI assistant."

        # @prompt
        self.user_greeting = f"Hello {self.name}!"

        self.regular_var = 42

    def process(self, message: str):
        # @prompt
        context_prompt = f"""
        You are processing: {message}
        Please respond accordingly.
        """

        response_template = f"Response to {message}"

        return self.generate(context_prompt)

    def generate(self, prompt):
        # @prompt
        system = "Generate a response"
        # @prompt
        user = f"Based on: {prompt}"

        return self.model.run(system, user)

# @prompt
global_prompt = "This is a global prompt"

def helper():
    local_prompt = f"Local {value}"
    return local_prompt
"#;

// Large code sample - complex nested structures
const LARGE_PYTHON: &str = r#"
from typing import List, Dict, Optional
import asyncio

class AIAssistant:
    """Advanced AI assistant with multiple prompt templates."""

    def __init__(self, config: Dict[str, str]):
        # @prompt
        self.system_prompt: str
        self.system_prompt = f"You are {config['role']} with {config['capabilities']}."

        # @prompt
        self.greeting_template = """
        Hello! I'm your AI assistant.
        I can help you with various tasks.
        """

        self.user_prefs = config

    async def process_query(self, query: str, context: Optional[str] = None):
        # @prompt
        base_prompt = f"Process query: {query}"

        if context:
            # @prompt
            contextualized_prompt = f"""
            Query: {query}
            Context: {context}
            Please provide a detailed response.
            """
            prompt_to_use = contextualized_prompt
        else:
            prompt_to_use = base_prompt

        return await self.generate_response(prompt_to_use)

    async def generate_response(self, prompt: str):
        # @prompt
        system_instruction = "Generate a helpful and accurate response"
        # @prompt
        user_message = f"Based on: {prompt}"

        result = await self.model.complete(system_instruction, user_message)
        return self.post_process(result)

    def post_process(self, text: str) -> str:
        # Non-prompt processing
        cleaned = text.strip()
        return cleaned

    @classmethod
    def create_from_config(cls, config_path: str):
        config = load_config(config_path)
        return cls(config)

class ConversationManager:
    def __init__(self):
        # @prompt
        self.conversation_start = "Let's begin our conversation."
        self.history: List[str] = []

    def add_message(self, message: str):
        self.history.append(message)

        if len(self.history) > 10:
            # @prompt
            summary_prompt = f"""
            Summarize the following conversation:
            {chr(10).join(self.history)}
            """
            self.summarize(summary_prompt)

    def summarize(self, prompt: str):
        # @prompt
        system = "You are a conversation summarizer"
        # @prompt
        user = f"Summarize: {prompt}"

        return generate_summary(system, user)

# @prompt
DEFAULT_SYSTEM_PROMPT = "You are a helpful AI assistant"

# @prompt
FALLBACK_PROMPT = f"""
If no other prompt is available,
use this fallback: {DEFAULT_SYSTEM_PROMPT}
"""

def create_specialized_prompt(task: str, domain: str):
    # @prompt
    specialized = f"""
    Task: {task}
    Domain: {domain}
    Please provide expert-level assistance.
    """
    return specialized

async def main():
    assistant = AIAssistant({'role': 'helper', 'capabilities': 'general'})
    response = await assistant.process_query("What is Python?")
    print(response)
"#;

fn bench_python_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("python_parsers");

    // Small benchmark
    group.throughput(Throughput::Bytes(SMALL_PYTHON.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("RustPython", "small"),
        &SMALL_PYTHON,
        |b, code| {
            b.iter(|| {
                volumen_parser_py::ParserPy::parse(black_box(code), "test.py")
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Ruff", "small"),
        &SMALL_PYTHON,
        |b, code| {
            b.iter(|| {
                volumen_parser_py_ruff::ParserPy::parse(black_box(code), "test.py")
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "small"),
        &SMALL_PYTHON,
        |b, code| {
            b.iter(|| {
                volumen_parser_py_tree_sitter::ParserPy::parse(black_box(code), "test.py")
            });
        },
    );

    // Medium benchmark
    group.throughput(Throughput::Bytes(MEDIUM_PYTHON.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("RustPython", "medium"),
        &MEDIUM_PYTHON,
        |b, code| {
            b.iter(|| {
                volumen_parser_py::ParserPy::parse(black_box(code), "test.py")
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Ruff", "medium"),
        &MEDIUM_PYTHON,
        |b, code| {
            b.iter(|| {
                volumen_parser_py_ruff::ParserPy::parse(black_box(code), "test.py")
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "medium"),
        &MEDIUM_PYTHON,
        |b, code| {
            b.iter(|| {
                volumen_parser_py_tree_sitter::ParserPy::parse(black_box(code), "test.py")
            });
        },
    );

    // Large benchmark
    group.throughput(Throughput::Bytes(LARGE_PYTHON.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("RustPython", "large"),
        &LARGE_PYTHON,
        |b, code| {
            b.iter(|| {
                volumen_parser_py::ParserPy::parse(black_box(code), "test.py")
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Ruff", "large"),
        &LARGE_PYTHON,
        |b, code| {
            b.iter(|| {
                volumen_parser_py_ruff::ParserPy::parse(black_box(code), "test.py")
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "large"),
        &LARGE_PYTHON,
        |b, code| {
            b.iter(|| {
                volumen_parser_py_tree_sitter::ParserPy::parse(black_box(code), "test.py")
            });
        },
    );

    group.finish();
}

criterion_group!(benches, bench_python_parsers);
criterion_main!(benches);
