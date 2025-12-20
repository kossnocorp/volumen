use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use volumen_parser_core::VolumenParser;

// Small code sample - basic prompt detection
const SMALL_RUBY: &str = r#"
# @prompt
user_prompt = "You are a helpful assistant."
system_prompt = "You are a #{role}."
"#;

// Medium code sample - multiple prompts with annotations
const MEDIUM_RUBY: &str = r#"
class ChatBot
  def initialize
    # @prompt
    @system_prompt = "You are a helpful AI assistant."

    # @prompt
    @user_greeting = "Hello #{@name}!"

    @regular_var = 42
  end

  def process(message)
    # @prompt
    context_prompt = <<~HEREDOC
      You are processing: #{message}
      Please respond accordingly.
    HEREDOC

    response_template = "Response to #{message}"

    generate(context_prompt)
  end

  def generate(prompt)
    # @prompt
    system = "Generate a response"
    # @prompt
    user = "Based on: #{prompt}"

    @model.run(system, user)
  end
end

# @prompt
global_prompt = "This is a global prompt"

def helper
  local_prompt = "Local #{value}"
  local_prompt
end
"#;

// Large code sample - complex nested structures
const LARGE_RUBY: &str = r#"
require_relative 'types'

class AIAssistant
  attr_reader :system_prompt, :greeting_template, :user_prefs

  def initialize(config)
    # @prompt
    @system_prompt = "You are #{config[:role]} with #{config[:capabilities]}."

    # @prompt
    @greeting_template = <<~HEREDOC
      Hello! I'm your AI assistant.
      I can help you with various tasks.
    HEREDOC

    @user_prefs = config
    @model = ModelClient.new
  end

  def process_query(query, context: nil)
    # @prompt
    base_prompt = "Process query: #{query}"

    prompt_to_use = if context
      # @prompt
      contextualized_prompt = <<~HEREDOC
        Query: #{query}
        Context: #{context}
        Please provide a detailed response.
      HEREDOC
      contextualized_prompt
    else
      base_prompt
    end

    generate_response(prompt_to_use)
  end

  def generate_response(prompt)
    # @prompt
    system_instruction = "Generate a helpful and accurate response"
    # @prompt
    user_message = "Based on: #{prompt}"

    result = @model.complete(system_instruction, user_message)
    post_process(result)
  end

  def post_process(text)
    # Non-prompt processing
    text.strip
  end

  def self.create_from_config(config_path)
    config = load_config(config_path)
    new(config)
  end
end

class ConversationManager
  def initialize
    # @prompt
    @conversation_start = "Let's begin our conversation."
    @history = []
  end

  def add_message(message)
    @history << message

    if @history.length > 10
      # @prompt
      summary_prompt = <<~HEREDOC
        Summarize the following conversation:
        #{@history.join("\n")}
      HEREDOC
      summarize(summary_prompt)
    end
  end

  def summarize(prompt)
    # @prompt
    system = "You are a conversation summarizer"
    # @prompt
    user = "Summarize: #{prompt}"

    generate_summary(system, user)
  end
end

# @prompt
DEFAULT_SYSTEM_PROMPT = "You are a helpful AI assistant"

# @prompt
FALLBACK_PROMPT = <<~HEREDOC
  If no other prompt is available,
  use this fallback: #{DEFAULT_SYSTEM_PROMPT}
HEREDOC

def create_specialized_prompt(task, domain)
  # @prompt
  specialized = <<~HEREDOC
    Task: #{task}
    Domain: #{domain}
    Please provide expert-level assistance.
  HEREDOC
  specialized
end

def main
  assistant = AIAssistant.new(
    role: 'helper',
    capabilities: 'general'
  )

  response = assistant.process_query("What is Ruby?")
  puts response
end
"#;

fn bench_ruby_parsers(c: &mut Criterion) {
    let mut group = c.benchmark_group("ruby_parsers");

    // Small benchmark
    group.throughput(Throughput::Bytes(SMALL_RUBY.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "small"),
        &SMALL_RUBY,
        |b, code| {
            b.iter(|| volumen_parser_rb::ParserRb::parse(black_box(code), "test.rb"));
        },
    );

    // Medium benchmark
    group.throughput(Throughput::Bytes(MEDIUM_RUBY.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "medium"),
        &MEDIUM_RUBY,
        |b, code| {
            b.iter(|| volumen_parser_rb::ParserRb::parse(black_box(code), "test.rb"));
        },
    );

    // Large benchmark
    group.throughput(Throughput::Bytes(LARGE_RUBY.len() as u64));

    group.bench_with_input(
        BenchmarkId::new("Tree-sitter", "large"),
        &LARGE_RUBY,
        |b, code| {
            b.iter(|| volumen_parser_rb::ParserRb::parse(black_box(code), "test.rb"));
        },
    );

    group.finish();
}

criterion_group!(benches, bench_ruby_parsers);
criterion_main!(benches);
