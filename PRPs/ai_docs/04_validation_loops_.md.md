# Chapter 4: Validation Loops

Welcome back! In [Chapter 3: PRP (Product Requirement Prompt)](03_prp__product_requirement_prompt__.md), you learned that a PRP is a detailed work order for an AI agent, containing not just *what* to build, but also the necessary context and guidance on *how* to build it. You saw that a crucial section in the PRP is the `## Validation Loop`.

In this chapter, we'll focus entirely on this vital concept: **Validation Loops**. They are the secret sauce that allows the AI to build reliable code and fix its own mistakes.

## The Problem: How Does the AI Know If It's Right?

Imagine you've given the AI a PRP to build that user profile endpoint we discussed in previous chapters. The AI writes some code. Great! But is the code correct?

*   Does it follow the project's code style rules?
*   Does it have any basic syntax errors or type mismatches?
*   Does it actually work as intended? Does the endpoint return the correct data?
*   Did the new code break any existing features?

As a human engineer, you'd run checks: you'd run a linter, a type checker, and execute unit or integration tests. If any of these failed, you'd get error messages, figure out the problem, fix your code, and run the checks again. You repeat this cycle until all checks pass.

An AI agent needs to do the same thing! It can write code, but without a way to automatically verify its work, it's just guessing if the code is correct. This is where Validation Loops come in.

## What Are Validation Loops?

**Validation Loops** are executable checks that you embed directly within the PRP document. They are specific commands (usually bash commands) that the AI agent is instructed to run after it makes a code change or completes a step.

Think of a Validation Loop as a **self-correction mechanism** built into the AI's workflow.

Here's the core idea:

1.  The AI receives the PRP and understands the task and the validation commands required.
2.  The AI writes some code based on the PRP's blueprint.
3.  The AI *runs the specified validation commands*.
4.  If the commands **pass** (exit with status 0), the AI knows that part of its work is likely correct and can move on.
5.  If the commands **fail** (exit with a non-zero status, like a test failing or a linter reporting errors), the AI receives the error output as feedback.
6.  The AI **analyzes the error output**, understands what went wrong, goes back to the code, and attempts to fix the identified issues.
7.  The AI then **repeats** steps 2-6 (write/fix code -> run checks -> analyze -> fix) until the validation commands pass.

This loop of coding, validating, analyzing feedback, and fixing is the "Validation Loop." It allows the AI to autonomously identify and correct its own errors, dramatically increasing the chances of getting working, high-quality code on the first "pass" from your perspective.

## Our Use Case: Ensuring the User Profile Endpoint Works and is Correctly Styled

Let's stick with our user profile endpoint example. How do we use Validation Loops to ensure the AI builds it correctly?

In the PRP document for this feature (`PRPs/implement-user-profile.md`), you would include a `## Validation Loop` section. Inside this section, you list the specific commands the AI must run.

For our Python project example, these might include:

*   A command to check code style and syntax (like `ruff check`).
*   A command to check type hints (like `mypy`).
*   A command to run unit tests for the new endpoint code.
*   A command to run an integration test (like a `curl` request) to see if the endpoint responds correctly.

When you execute this PRP using the `/project:PRPs:prp-base-execute` command (as learned in Chapter 2), the AI will implement the code, and then systematically run these commands. If `ruff` finds a style error, the AI will read the error message, modify the code to fix the style, and run `ruff` again. If a unit test fails, the AI will read the test failure output, debug the code, and re-run the tests. This continues until all checks pass.

## Structure of the Validation Loop in a PRP

Looking at the `PRPs/templates/prp_base.md` file, you'll see the `## Validation Loop` section contains executable bash code blocks.

```markdown
## Validation Loop

### Level 1: Syntax & Style

```bash
# Run these FIRST - fix any errors before proceeding
ruff check src/ --fix  # Auto-fix what's possible across src/
mypy src/              # Type checking across src/

# Expected: No errors. If errors, READ the error and fix.
```

### Level 2: Unit Tests each new feature/file/function use existing test patterns

```bash
# Run and iterate until passing:
uv run pytest tests/users/test_api.py -v
# If failing: Read error, understand root cause, fix code, re-run (never mock to pass)
```

### Level 3: Integration Test

```bash
# Start the service (this might be done by a separate process or command)
# uv run python -m src.main --dev

# Test the endpoint
curl -X GET http://localhost:8000/users/1 \
  -H "Accept: application/json"

# Expected: A JSON response with user data or a 404 error.
# If error: Check logs at logs/app.log for stack trace
```

These sections are crucial because they provide the AI with concrete, objective criteria for success. The AI isn't just hoping the code works; it's running the same automated checks you would, and it uses the output of those checks as its primary feedback loop.

The 'levels' (Level 1, Level 2, etc.) suggested in the template provide a logical order: fix basic syntax/style first (Level 1), then ensure individual components work (Level 2 Unit Tests), then check if different parts work together (Level 3 Integration Tests). You can customize these levels and commands based on your project's setup.

## Why Are Validation Loops Critical for Agentic Engineering?

Validation loops are a cornerstone of the PRP methodology and agentic engineering for several key reasons:

1.  **Self-Correction:** This is the most important benefit. The AI doesn't require you to manually review *every* code change and point out errors. It finds and fixes many issues itself.
2.  **Deterministic Feedback:** Unlike subjective instructions ("make the code better"), running tests or linters provides objective, clear pass/fail signals and detailed error messages. This is ideal feedback for an AI to process.
3.  **Increased Autonomy:** With validation loops, the AI can work on more complex tasks without constant human intervention. You can give it the PRP and trust it to iterate until the defined criteria are met.
4.  **Higher Quality Output:** By enforcing style guides, type checks, and passing tests automatically, the AI is pushed towards producing higher-quality, more reliable code from the outset.
5.  **Faster Iteration:** The AI can run these checks much faster than a human could manually review and test every small change.

They are what allow the AI to move beyond simply generating code snippets to reliably implementing features that integrate correctly and meet quality standards.

## Under the Hood: The Loop in Action (Simplified)

When the AI executes a PRP with a Validation Loop via the `/project:PRPs:prp-base-execute` command, here's a simplified view of what happens during the validation phase:

```mermaid
sequenceDiagram
    participant User
    participant ClaudeCodeTool
    participant ClaudeAPI
    participant BashShell

    User->>ClaudeCodeTool: Run /execute-base-prp my-prp.md
    ClaudeCodeTool->>ClaudeAPI: Send PRP content + Execution instructions
    ClaudeAPI->>ClaudeAPI: (Plan, Write Code)
    loop Validation Loop (until pass)
        ClaudeAPI->>ClaudeCodeTool: Instruct to run validation commands (from PRP)
        ClaudeCodeTool->>BashShell: Execute ruff check, mypy, pytest etc.
        BashShell-->>ClaudeCodeTool: Return command output (errors/success)
        ClaudeCodeTool-->>ClaudeAPI: Send Validation Results & Output
        alt Commands Failed
            ClaudeAPI->>ClaudeAPI: Analyze output, identify errors
            ClaudeAPI->>ClaudeAPI: Plan code fixes
            ClaudeAPI->>ClaudeAPI: Implement fixes (Modify Code)
        else Commands Passed
            break
        end
    end
    ClaudeAPI->>ClaudeAPI: (Final Checks, Report Completion)
    ClaudeAPI-->>ClaudeCodeTool: Report Completion
    ClaudeCodeTool->>User: Display Completion Status
```

The Claude Code tool acts as the intermediary, allowing the Claude API model to instruct the tool to run commands in your local environment (the BashShell). The output is then fed back to the API model, closing the loop and enabling the AI to self-correct.

This process repeats until all the validation commands specified in the PRP's `## Validation Loop` section pass successfully.

## Conclusion

In this chapter, you learned that **Validation Loops** are executable checks embedded within the PRP document. They are essential for agentic engineering because they provide the AI with a concrete way to verify its own work.

By listing specific commands like linters, type checkers, and tests in the `## Validation Loop` section of your PRP, you empower the AI to run these checks, analyze the results, and automatically fix errors until all validations pass. This critical mechanism enables self-correction, increases AI autonomy, and leads to higher-quality code output.

You now understand that when you run `/project:PRPs:prp-base-execute`, you are not just telling the AI to code; you're giving it a detailed recipe that includes the crucial steps of building, validating, and iterating until the code meets your defined, executable standards.

In the next chapter, we'll shift gears slightly and look at **[PRP Creation (Generating a PRP)](05_prp_creation__generating_a_prp__.md)** – how you can leverage the AI itself to help you write these detailed PRP documents in the first place.

[PRP Creation (Generating a PRP)](05_prp_creation__generating_a_prp__.md)

---

<sub><sup>Generated by [AI Codebase Knowledge Builder](https://github.com/The-Pocket/Tutorial-Codebase-Knowledge).</sup></sub> <sub><sup>**References**: [[1]](https://github.com/Wirasm/PRPs-agentic-eng/blob/57205a3f8360e7ba23bac76df6bca9d200ec3b6e/.claude/commands/rapid-development/experimental/prp-validate.md), [[2]](https://github.com/Wirasm/PRPs-agentic-eng/blob/57205a3f8360e7ba23bac76df6bca9d200ec3b6e/CLAUDE.md), [[3]](https://github.com/Wirasm/PRPs-agentic-eng/blob/57205a3f8360e7ba23bac76df6bca9d200ec3b6e/PRPs/README.md), [[4]](https://github.com/Wirasm/PRPs-agentic-eng/blob/57205a3f8360e7ba23bac76df6bca9d200ec3b6e/PRPs/templates/prp_base.md), [[5]](https://github.com/Wirasm/PRPs-agentic-eng/blob/57205a3f8360e7ba23bac76df6bca9d200ec3b6e/README.md)</sup></sub>
````