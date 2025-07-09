# Create BASE PRP for cs-professor-agent

## Feature: $ARGUMENTS

Generate a PRP for a feature in the Mastra + AG-UI agentic project, following the PRP Method. Use the @prp_base.md template and ensure all context is included for one-pass implementation.

## Research Process

1. **Codebase Analysis**
   - Review existing agent, tool, and UI patterns in `cs-professor-agent/` (see README.md, TASK.md, PLANNING.md).
   - Reference all docs in `/PRPs-agentic-eng/PRPs/ai_docs/mastra` like the file '@SUPABASE_SETUP.md' for implementation context:
     - Model_Providers_Mastra.md
     - PG_vector_store.md
     - PostgreSQL_storage.md
     - RAG_Overview_MASTRA.md
     - RAG_Research_Assistant_on_MASTRA.md
     - SUPABASE_SETUP.md
     - vectorDatabase_Mastra.md
   - Identify conventions for Mastra, AG-UI, RAG, and HITL.

2. **External Research**
   - Link to Mastra, CopilotKit, AG-UI, Supabase, and E2B docs as needed.
   - Reference best practices for agentic architectures and context engineering.

3. **User Clarification**
   - Confirm integration points, validation gates, and UI/UX requirements.

## PRP Generation

- Use `PRPs/templates/prp_base.md` (@prp_base.md) as the template.
- In "Documentation & References", list all files in `/PRPs-agentic-eng/PRPs/ai_docs/mastra` and any relevant URLs.
- Include codebase tree, desired file changes, and known gotchas.
- Provide implementation blueprint, task list, and per-task pseudocode.
- Add validation gates (lint, typecheck, tests, manual validation).
- Complete the quality checklist and score confidence.

## Output

Save as: `PRPs/{feature-name}.md`
