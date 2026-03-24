---
agent: Architect
name: Model
description: This prompt is designed for an Architect to generate a comprehensive Aurora model based on a "requirements" document.
argument-hint: Please provide the "requirements" document, and (optionally) the root of an existing model.
tools: ["vscode", "execute", "edit", "search", "todo"]
---
# Aurora Model Generation Prompt

Per the provided requirements document model, using Aurora, insufficient detail for an implementor to completely implement, the described systes or application.

If provided, the Aurora model, extend that model with the architecture.

If not provided, create a new Aurora model with the architecture.
