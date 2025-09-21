use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Programming languages supported by the judger
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgrammingLanguage {
    C,
    Cpp,
    Cpp11,
    Cpp14,
    Cpp17,
    Cpp20,
    Python2,
    Python3,
    Java,
    Rust,
    Go,
    JavaScript,
    TypeScript,
}

impl ProgrammingLanguage {
    /// Returns the file extension for this language
    pub fn file_extension(&self) -> &'static str {
        match self {
            ProgrammingLanguage::C => "c",
            ProgrammingLanguage::Cpp => "cpp",
            ProgrammingLanguage::Cpp11 => "cpp",
            ProgrammingLanguage::Cpp14 => "cpp",
            ProgrammingLanguage::Cpp17 => "cpp",
            ProgrammingLanguage::Cpp20 => "cpp",
            ProgrammingLanguage::Python2 => "py",
            ProgrammingLanguage::Python3 => "py",
            ProgrammingLanguage::Java => "java",
            ProgrammingLanguage::Rust => "rs",
            ProgrammingLanguage::Go => "go",
            ProgrammingLanguage::JavaScript => "js",
            ProgrammingLanguage::TypeScript => "ts",
        }
    }

    /// Returns the default compiler/interpreter command for this language
    pub fn default_compiler(&self) -> &'static str {
        match self {
            ProgrammingLanguage::C => "gcc",
            ProgrammingLanguage::Cpp => "g++",
            ProgrammingLanguage::Cpp11 => "g++",
            ProgrammingLanguage::Cpp14 => "g++",
            ProgrammingLanguage::Cpp17 => "g++",
            ProgrammingLanguage::Cpp20 => "g++",
            ProgrammingLanguage::Python2 => "python2",
            ProgrammingLanguage::Python3 => "python3",
            ProgrammingLanguage::Java => "javac",
            ProgrammingLanguage::Rust => "rustc",
            ProgrammingLanguage::Go => "go",
            ProgrammingLanguage::JavaScript => "node",
            ProgrammingLanguage::TypeScript => "ts-node",
        }
    }

    /// Returns whether this language needs compilation
    pub fn needs_compilation(&self) -> bool {
        match self {
            ProgrammingLanguage::C
            | ProgrammingLanguage::Cpp
            | ProgrammingLanguage::Cpp11
            | ProgrammingLanguage::Cpp14
            | ProgrammingLanguage::Cpp17
            | ProgrammingLanguage::Cpp20
            | ProgrammingLanguage::Java
            | ProgrammingLanguage::Rust
            | ProgrammingLanguage::Go => true,
            ProgrammingLanguage::Python2
            | ProgrammingLanguage::Python3
            | ProgrammingLanguage::JavaScript
            | ProgrammingLanguage::TypeScript => false,
        }
    }

    /// Returns the default compilation flags for this language
    pub fn default_compile_flags(&self) -> Vec<String> {
        match self {
            ProgrammingLanguage::C => vec!["-O2".to_string(), "-Wall".to_string()],
            ProgrammingLanguage::Cpp => vec![
                "-O2".to_string(),
                "-Wall".to_string(),
                "-std=c++11".to_string(),
            ],
            ProgrammingLanguage::Cpp11 => vec![
                "-O2".to_string(),
                "-Wall".to_string(),
                "-std=c++11".to_string(),
            ],
            ProgrammingLanguage::Cpp14 => vec![
                "-O2".to_string(),
                "-Wall".to_string(),
                "-std=c++14".to_string(),
            ],
            ProgrammingLanguage::Cpp17 => vec![
                "-O2".to_string(),
                "-Wall".to_string(),
                "-std=c++17".to_string(),
            ],
            ProgrammingLanguage::Cpp20 => vec![
                "-O2".to_string(),
                "-Wall".to_string(),
                "-std=c++20".to_string(),
            ],
            ProgrammingLanguage::Java => vec!["-Xlint:all".to_string()],
            ProgrammingLanguage::Rust => vec!["-O".to_string()],
            ProgrammingLanguage::Go => vec![],
            _ => vec![],
        }
    }

    /// Returns the default runtime command for this language
    pub fn default_runtime(&self) -> &'static str {
        match self {
            ProgrammingLanguage::C => "./a.out",
            ProgrammingLanguage::Cpp => "./a.out",
            ProgrammingLanguage::Cpp11 => "./a.out",
            ProgrammingLanguage::Cpp14 => "./a.out",
            ProgrammingLanguage::Cpp17 => "./a.out",
            ProgrammingLanguage::Cpp20 => "./a.out",
            ProgrammingLanguage::Python2 => "python2",
            ProgrammingLanguage::Python3 => "python3",
            ProgrammingLanguage::Java => "java",
            ProgrammingLanguage::Rust => "./main",
            ProgrammingLanguage::Go => "./main",
            ProgrammingLanguage::JavaScript => "node",
            ProgrammingLanguage::TypeScript => "ts-node",
        }
    }

    /// Returns a string representation of the language
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgrammingLanguage::C => "C",
            ProgrammingLanguage::Cpp => "C++",
            ProgrammingLanguage::Cpp11 => "C++11",
            ProgrammingLanguage::Cpp14 => "C++14",
            ProgrammingLanguage::Cpp17 => "C++17",
            ProgrammingLanguage::Cpp20 => "C++20",
            ProgrammingLanguage::Python2 => "Python 2",
            ProgrammingLanguage::Python3 => "Python 3",
            ProgrammingLanguage::Java => "Java",
            ProgrammingLanguage::Rust => "Rust",
            ProgrammingLanguage::Go => "Go",
            ProgrammingLanguage::JavaScript => "JavaScript",
            ProgrammingLanguage::TypeScript => "TypeScript",
        }
    }
}

/// Submission information for judging
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Submission {
    /// Unique identifier for the submission
    pub id: Uuid,
    /// Problem identifier
    pub problem_id: Uuid,
    /// User identifier who submitted the code
    pub user_id: Uuid,
    /// Programming language of the submission
    pub language: ProgrammingLanguage,
    /// Source code to be judged
    pub source_code: String,
    /// Time when the submission was created
    pub created_at: DateTime<Utc>,
    /// Time limit in milliseconds
    pub time_limit: u64,
    /// Memory limit in kilobytes
    pub memory_limit: u64,
    /// Judge priority (higher numbers get processed first)
    pub priority: i32,
    /// Contest identifier if this is a contest submission
    pub contest_id: Option<Uuid>,
}

impl Submission {
    /// Creates a new submission with default values
    pub fn new(
        problem_id: Uuid,
        user_id: Uuid,
        language: ProgrammingLanguage,
        source_code: String,
        time_limit: u64,
        memory_limit: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            problem_id,
            user_id,
            language,
            source_code,
            created_at: Utc::now(),
            time_limit,
            memory_limit,
            priority: 0,
            contest_id: None,
        }
    }

    /// Creates a contest submission
    pub fn for_contest(
        problem_id: Uuid,
        user_id: Uuid,
        contest_id: Uuid,
        language: ProgrammingLanguage,
        source_code: String,
        time_limit: u64,
        memory_limit: u64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            problem_id,
            user_id,
            language,
            source_code,
            created_at: Utc::now(),
            time_limit,
            memory_limit,
            priority: 10, // Higher priority for contest submissions
            contest_id: Some(contest_id),
        }
    }

    /// Returns the filename for this submission based on language
    pub fn filename(&self) -> String {
        format!("main.{}", self.language.file_extension())
    }

    /// Returns whether this submission needs compilation
    pub fn needs_compilation(&self) -> bool {
        self.language.needs_compilation()
    }

    /// Returns the default compilation flags for this submission's language
    pub fn default_compile_flags(&self) -> Vec<String> {
        self.language.default_compile_flags()
    }

    /// Returns the default runtime command for this submission's language
    pub fn default_runtime(&self) -> &'static str {
        self.language.default_runtime()
    }
}

/// Judge task for the judger service
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JudgeTask {
    /// Submission information
    pub submission: Submission,
    /// Test cases to run against the submission
    pub test_cases: Vec<TestCase>,
    /// Whether to compile the code (true for compiled languages)
    pub needs_compilation: bool,
    /// Whether to run the code in a sandbox
    pub use_sandbox: bool,
    /// Additional compilation flags
    pub compile_flags: Option<Vec<String>>,
    /// Additional runtime arguments
    pub runtime_args: Option<Vec<String>>,
}

impl JudgeTask {
    /// Creates a new judge task from a submission and test cases
    pub fn new(submission: Submission, test_cases: Vec<TestCase>) -> Self {
        let needs_compilation = submission.needs_compilation();
        let compile_flags = if needs_compilation {
            Some(submission.default_compile_flags())
        } else {
            None
        };

        Self {
            submission,
            test_cases,
            needs_compilation,
            use_sandbox: true, // Always use sandbox for security
            compile_flags,
            runtime_args: None,
        }
    }

    /// Returns the number of test cases in this task
    pub fn test_case_count(&self) -> usize {
        self.test_cases.len()
    }

    /// Returns the total weight of all test cases
    pub fn total_weight(&self) -> f64 {
        self.test_cases.iter().map(|tc| tc.weight).sum()
    }

    /// Returns the maximum time limit among all test cases
    pub fn max_time_limit(&self) -> u64 {
        self.test_cases
            .iter()
            .map(|tc| tc.time_limit.unwrap_or(self.submission.time_limit))
            .max()
            .unwrap_or(self.submission.time_limit)
    }

    /// Returns the maximum memory limit among all test cases
    pub fn max_memory_limit(&self) -> u64 {
        self.test_cases
            .iter()
            .map(|tc| tc.memory_limit.unwrap_or(self.submission.memory_limit))
            .max()
            .unwrap_or(self.submission.memory_limit)
    }
}

/// Test case definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCase {
    /// Test case identifier
    pub id: String,
    /// Input data for the test case
    pub input: String,
    /// Expected output
    pub expected_output: String,
    /// Time limit for this specific test case (overrides submission time limit)
    pub time_limit: Option<u64>,
    /// Memory limit for this specific test case (overrides submission memory limit)
    pub memory_limit: Option<u64>,
    /// Whether this test case is hidden (not shown to user)
    pub is_hidden: bool,
    /// Weight of this test case in scoring
    pub weight: f64,
}

impl TestCase {
    /// Creates a new test case with default values
    pub fn new(id: String, input: String, expected_output: String) -> Self {
        Self {
            id,
            input,
            expected_output,
            time_limit: None,
            memory_limit: None,
            is_hidden: false,
            weight: 1.0,
        }
    }

    /// Creates a hidden test case
    pub fn hidden(id: String, input: String, expected_output: String) -> Self {
        Self {
            id,
            input,
            expected_output,
            time_limit: None,
            memory_limit: None,
            is_hidden: true,
            weight: 1.0,
        }
    }

    /// Creates a test case with custom limits
    pub fn with_limits(
        id: String,
        input: String,
        expected_output: String,
        time_limit: u64,
        memory_limit: u64,
    ) -> Self {
        Self {
            id,
            input,
            expected_output,
            time_limit: Some(time_limit),
            memory_limit: Some(memory_limit),
            is_hidden: false,
            weight: 1.0,
        }
    }

    /// Returns the effective time limit (custom or default)
    pub fn effective_time_limit(&self, default_time_limit: u64) -> u64 {
        self.time_limit.unwrap_or(default_time_limit)
    }

    /// Returns the effective memory limit (custom or default)
    pub fn effective_memory_limit(&self, default_memory_limit: u64) -> u64 {
        self.memory_limit.unwrap_or(default_memory_limit)
    }
}

/// Detailed information about a judgment result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JudgeResult {
    /// The overall status of the judgment
    pub status: JudgeStatus,
    /// Time consumed in milliseconds
    pub time_used: u64,
    /// Memory used in kilobytes
    pub memory_used: u64,
    /// Detailed error information if applicable
    pub error_info: Option<ErrorInfo>,
    /// Test case results (for multi-test case judgments)
    pub test_cases: Vec<TestCaseResult>,
    /// Submission ID this result belongs to
    pub submission_id: Uuid,
    /// Problem ID
    pub problem_id: Uuid,
    /// User ID
    pub user_id: Uuid,
    /// When the judgment was completed
    pub judged_at: DateTime<Utc>,
    /// Score achieved (0.0 to 100.0)
    pub score: f64,
}

/// Represents the status of a code submission judgment with detailed variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JudgeStatus {
    /// The submission passed all test cases
    Accepted,
    /// The submission failed one or more test cases
    WrongAnswer,
    /// The submission exceeded the time limit
    TimeLimitExceeded,
    /// The submission exceeded the memory limit
    MemoryLimitExceeded,
    /// The submission encountered a runtime error
    RuntimeError(RuntimeErrorType),
    /// The submission failed to compile
    CompileError,
    /// The submission contains restricted operations
    RestrictedOperation,
    /// Output limit exceeded
    OutputLimitExceeded,
    /// An internal system error occurred during judgment
    SystemError,
    /// The submission is waiting in queue
    Pending,
    /// The submission is currently being judged
    Judging,
    /// The submission was cancelled
    Cancelled,
}

/// Types of runtime errors that can occur
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeErrorType {
    /// Segmentation fault
    SegmentationFault,
    /// Floating point exception
    FloatingPointException,
    /// Division by zero
    DivisionByZero,
    /// Assertion failed
    AssertionFailed,
    /// Stack overflow
    StackOverflow,
    /// Null pointer dereference
    NullPointerDereference,
    /// File operation error
    FileOperationError,
    /// Permission denied
    PermissionDenied,
    /// Other unspecified runtime error
    Other,
}

/// Detailed error information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Error message
    pub message: String,
    /// Error code (if applicable)
    pub code: Option<String>,
    /// Line number where error occurred
    pub line: Option<u32>,
    /// Column number where error occurred
    pub column: Option<u32>,
    /// Standard error output
    pub stderr: Option<String>,
    /// Standard output
    pub stdout: Option<String>,
    /// Exit code of the process
    pub exit_code: Option<i32>,
    /// Signal that terminated the process
    pub signal: Option<i32>,
}

/// Result for an individual test case
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestCaseResult {
    /// Test case identifier
    pub id: String,
    /// Status for this test case
    pub status: JudgeStatus,
    /// Time used for this test case (ms)
    pub time_used: u64,
    /// Memory used for this test case (KB)
    pub memory_used: u64,
    /// Input data for the test case
    pub input: Option<String>,
    /// Expected output
    pub expected_output: Option<String>,
    /// Actual output from the submission
    pub actual_output: Option<String>,
    /// Error information if the test case failed
    pub error_info: Option<ErrorInfo>,
}

impl JudgeStatus {
    /// Returns true if the status represents a successful submission
    pub fn is_accepted(&self) -> bool {
        matches!(self, JudgeStatus::Accepted)
    }

    /// Returns true if the status represents a final state (not pending or judging)
    pub fn is_final(&self) -> bool {
        !matches!(self, JudgeStatus::Pending | JudgeStatus::Judging)
    }

    /// Returns true if the status represents an error (not including WrongAnswer)
    pub fn is_error(&self) -> bool {
        matches!(
            self,
            JudgeStatus::TimeLimitExceeded
                | JudgeStatus::MemoryLimitExceeded
                | JudgeStatus::RuntimeError(_)
                | JudgeStatus::CompileError
                | JudgeStatus::RestrictedOperation
                | JudgeStatus::OutputLimitExceeded
                | JudgeStatus::SystemError
        )
    }

    /// Returns true if the status represents a runtime error
    pub fn is_runtime_error(&self) -> bool {
        matches!(self, JudgeStatus::RuntimeError(_))
    }

    /// Returns a string representation of the status
    pub fn as_str(&self) -> &'static str {
        match self {
            JudgeStatus::Accepted => "Accepted",
            JudgeStatus::WrongAnswer => "Wrong Answer",
            JudgeStatus::TimeLimitExceeded => "Time Limit Exceeded",
            JudgeStatus::MemoryLimitExceeded => "Memory Limit Exceeded",
            JudgeStatus::RuntimeError(_) => "Runtime Error",
            JudgeStatus::CompileError => "Compile Error",
            JudgeStatus::RestrictedOperation => "Restricted Operation",
            JudgeStatus::OutputLimitExceeded => "Output Limit Exceeded",
            JudgeStatus::SystemError => {
                "System
 Error"
            }
            JudgeStatus::Pending => "Pending",
            JudgeStatus::Judging => "Judging",
            JudgeStatus::Cancelled => "Cancelled",
        }
    }

    /// Returns a short code for the status
    pub fn as_code(&self) -> &'static str {
        match self {
            JudgeStatus::Accepted => "AC",
            JudgeStatus::WrongAnswer => "WA",
            JudgeStatus::TimeLimitExceeded => "TLE",
            JudgeStatus::MemoryLimitExceeded => "MLE",
            JudgeStatus::RuntimeError(_) => "RE",
            JudgeStatus::CompileError => "CE",
            JudgeStatus::RestrictedOperation => "RO",
            JudgeStatus::OutputLimitExceeded => "OLE",
            JudgeStatus::SystemError => "SE",
            JudgeStatus::Pending => "PD",
            JudgeStatus::Judging => "JG",
            JudgeStatus::Cancelled => "CN",
        }
    }

    /// Returns the runtime error type if applicable
    pub fn runtime_error_type(&self) -> Option<RuntimeErrorType> {
        match self {
            JudgeStatus::RuntimeError(error_type) => Some(*error_type),
            _ => None,
        }
    }
}

impl RuntimeErrorType {
    /// Returns a string description of the runtime error
    pub fn as_str(&self) -> &'static str {
        match self {
            RuntimeErrorType::SegmentationFault => "Segmentation fault",
            RuntimeErrorType::FloatingPointException => "Floating point exception",
            RuntimeErrorType::DivisionByZero => "Division by zero",
            RuntimeErrorType::AssertionFailed => "Assertion failed",
            RuntimeErrorType::StackOverflow => "Stack overflow",
            RuntimeErrorType::NullPointerDereference => "Null pointer dereference",
            RuntimeErrorType::FileOperationError => "File operation error",
            RuntimeErrorType::PermissionDenied => "Permission denied",
            RuntimeErrorType::Other => "Unknown runtime error",
        }
    }
}

impl JudgeResult {
    /// Creates a new successful judgment result
    pub fn accepted(
        time_used: u64,
        memory_used: u64,
        submission_id: Uuid,
        problem_id: Uuid,
        user_id: Uuid,
    ) -> Self {
        Self {
            status: JudgeStatus::Accepted,
            time_used,
            memory_used,
            error_info: None,
            test_cases: Vec::new(),
            submission_id,
            problem_id,
            user_id,
            judged_at: Utc::now(),
            score: 100.0,
        }
    }

    /// Creates a new judgment result with error information
    pub fn with_error(
        status: JudgeStatus,
        time_used: u64,
        memory_used: u64,
        error_info: ErrorInfo,
        submission_id: Uuid,
        problem_id: Uuid,
        user_id: Uuid,
    ) -> Self {
        Self {
            status,
            time_used,
            memory_used,
            error_info: Some(error_info),
            test_cases: Vec::new(),
            submission_id,
            problem_id,
            user_id,
            judged_at: Utc::now(),
            score: 0.0,
        }
    }

    /// Adds a test case result to the judgment
    pub fn add_test_case(&mut self, test_case: TestCaseResult) {
        self.test_cases.push(test_case);
    }

    /// Returns the number of passed test cases
    pub fn passed_test_cases(&self) -> usize {
        self.test_cases
            .iter()
            .filter(|tc| tc.status.is_accepted())
            .count()
    }

    /// Returns the total number of test cases
    pub fn total_test_cases(&self) -> usize {
        self.test_cases.len()
    }
}

impl ErrorInfo {
    /// Creates a new error info with a message
    pub fn new(message: String) -> Self {
        Self {
            message,
            code: None,
            line: None,
            column: None,
            stderr: None,
            stdout: None,
            exit_code: None,
            signal: None,
        }
    }

    /// Creates error info from stderr content
    pub fn from_stderr(stderr: String) -> Self {
        Self {
            message: stderr.clone(),
            code: None,
            line: None,
            column: None,
            stderr: Some(stderr),
            stdout: None,
            exit_code: None,
            signal: None,
        }
    }

    /// Creates error info for a compilation error
    pub fn compilation_error(message: String, stderr: Option<String>) -> Self {
        Self {
            message,
            code: None,
            line: None,
            column: None,
            stderr,
            stdout: None,
            exit_code: None,
            signal: None,
        }
    }

    /// Creates error info for a runtime error with signal
    pub fn runtime_error(message: String, signal: i32, stderr: Option<String>) -> Self {
        Self {
            message,
            code: None,
            line: None,
            column: None,
            stderr,
            stdout: None,
            exit_code: None,
            signal: Some(signal),
        }
    }
}

impl fmt::Display for JudgeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for RuntimeErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<JudgeStatus> for String {
    fn from(status: JudgeStatus) -> String {
        status.as_str().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_examples() {
        // Example 1: Successful submission
        let submission_id = Uuid::new_v4();
        let problem_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let successful_result =
            JudgeResult::accepted(150, 1024, submission_id, problem_id, user_id);
        assert!(successful_result.status.is_accepted());
        assert_eq!(successful_result.time_used, 150);
        assert_eq!(successful_result.memory_used, 1024);

        // Example 2: Compilation error
        let compile_error = ErrorInfo::compilation_error(
            "error: expected ';' after expression".to_string(),
            Some("main.c:5:5: error: expected ';' after expression".to_string()),
        );
        let compile_result = JudgeResult::with_error(
            JudgeStatus::CompileError,
            0,
            0,
            compile_error,
            submission_id,
            problem_id,
            user_id,
        );
        assert!(compile_result.status.is_error());

        // Example 3: Runtime error with specific type
        let runtime_status = JudgeStatus::RuntimeError(RuntimeErrorType::SegmentationFault);
        let runtime_error = ErrorInfo::runtime_error(
            "Segmentation fault (core dumped)".to_string(),
            11,
            Some("".to_string()),
        );
        let runtime_result = JudgeResult::with_error(
            runtime_status,
            50,
            256,
            runtime_error,
            submission_id,
            problem_id,
            user_id,
        );
        assert!(runtime_result.status.is_runtime_error());

        // Example 4: Test case results
        let mut test_result = JudgeResult::accepted(200, 768, submission_id, problem_id, user_id);
        test_result.add_test_case(TestCaseResult {
            id: "test_1".to_string(),
            status: JudgeStatus::Accepted,
            time_used: 50,
            memory_used: 256,
            input: Some("1 2".to_string()),
            expected_output: Some("3".to_string()),
            actual_output: Some("3".to_string()),
            error_info: None,
        });
        test_result.add_test_case(TestCaseResult {
            id: "test_2".to_string(),
            status: JudgeStatus::WrongAnswer,
            time_used: 75,
            memory_used: 384,
            input: Some("5 7".to_string()),
            expected_output: Some("12".to_string()),
            actual_output: Some("13".to_string()),
            error_info: None,
        });
        assert_eq!(test_result.passed_test_cases(), 1);
        assert_eq!(test_result.total_test_cases(), 2);
    }

    #[test]
    fn test_judge_status_methods() {
        assert!(JudgeStatus::Accepted.is_accepted());
        assert!(JudgeStatus::Accepted.is_final());
        assert!(!JudgeStatus::Accepted.is_error());

        assert!(JudgeStatus::RuntimeError(RuntimeErrorType::SegmentationFault).is_error());
        assert!(JudgeStatus::RuntimeError(RuntimeErrorType::SegmentationFault).is_runtime_error());

        assert!(!JudgeStatus::Pending.is_final());
        assert!(!JudgeStatus::Judging.is_final());
    }

    #[test]
    fn test_runtime_error_type() {
        let status = JudgeStatus::RuntimeError(RuntimeErrorType::SegmentationFault);
        assert_eq!(
            status.runtime_error_type(),
            Some(RuntimeErrorType::SegmentationFault)
        );
        assert_eq!(
            RuntimeErrorType::SegmentationFault.as_str(),
            "Segmentation fault"
        );
    }

    #[test]
    fn test_programming_language() {
        let lang = ProgrammingLanguage::Cpp17;
        assert_eq!(lang.file_extension(), "cpp");
        assert_eq!(lang.default_compiler(), "g++");
        assert!(lang.needs_compilation());
        assert_eq!(lang.as_str(), "C++17");

        let python = ProgrammingLanguage::Python3;
        assert_eq!(python.file_extension(), "py");
        assert!(!python.needs_compilation());
        assert_eq!(python.default_runtime(), "python3");
    }

    #[test]
    fn test_judge_result() {
        let submission_id = Uuid::new_v4();
        let problem_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let _result = JudgeResult::accepted(100, 512, submission_id, problem_id, user_id);
        let result = JudgeResult::accepted(150, 1024, submission_id, problem_id, user_id);
        assert!(result.status.is_accepted());
        assert_eq!(result.time_used, 150);
        assert_eq!(result.memory_used, 1024);
        assert!(result.error_info.is_none());
        assert_eq!(result.submission_id, submission_id);
        assert_eq!(result.score, 100.0);

        let error_info = ErrorInfo::new("Compilation failed".to_string());
        let error_result = JudgeResult::with_error(
            JudgeStatus::CompileError,
            0,
            0,
            error_info,
            submission_id,
            problem_id,
            user_id,
        );
        assert!(error_result.status.is_error());
        assert!(error_result.error_info.is_some());
        assert_eq!(error_result.score, 0.0);
    }

    #[test]
    fn test_error_info_creation() {
        let stderr_error = ErrorInfo::from_stderr("error: expected identifier".to_string());
        assert!(stderr_error.stderr.is_some());

        let compile_error = ErrorInfo::compilation_error(
            "Compilation failed".to_string(),
            Some("error: syntax error".to_string()),
        );
        assert_eq!(compile_error.message, "Compilation failed");
        assert!(compile_error.stderr.is_some());

        let runtime_error = ErrorInfo::runtime_error(
            "Segmentation fault".to_string(),
            11,
            Some("core dumped".to_string()),
        );
        assert_eq!(runtime_error.signal, Some(11));
    }

    #[test]
    fn test_serialization() {
        let submission_id = Uuid::new_v4();
        let problem_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let status = JudgeStatus::RuntimeError(RuntimeErrorType::DivisionByZero);
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: JudgeStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, status);

        let result = JudgeResult::accepted(150, 1024, submission_id, problem_id, user_id);
        let json = serde_json::to_string(&result).unwrap();
        let deserialized: JudgeResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.status, result.status);
    }
}
