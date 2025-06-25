#!/usr/bin/env python3
"""
Simple code generator from SwarmSH semantic conventions.
80/20 implementation - minimal effort, maximum value.
"""

import yaml
import os
from pathlib import Path

def load_semantic_conventions():
    """Load all SwarmSH semantic conventions."""
    semconv_dir = Path("semantic-conventions")
    conventions = {}
    
    # Load registry manifest
    with open(semconv_dir / "registry_manifest.yaml") as f:
        registry = yaml.safe_load(f)
    
    # Load all convention files
    for group in registry["groups"]:
        for path in group["paths"]:
            with open(semconv_dir / path) as f:
                data = yaml.safe_load(f)
                conventions[path] = data
    
    return conventions

def generate_attributes_rs(conventions):
    """Generate attributes.rs with all constants."""
    lines = [
        "// Generated from SwarmSH v2 semantic conventions",
        "// 80/20 implementation - core attributes only",
        "",
        "use std::collections::HashMap;",
        "",
    ]
    
    # Generate constants for each group
    for file_path, data in conventions.items():
        if "groups" not in data:
            continue
            
        for group in data["groups"]:
            group_id = group["id"].replace(".", "_").upper()
            lines.append(f"// {group.get('brief', 'Group constants')}")
            lines.append(f"pub mod {group_id.lower()} {{")
            
            if "attributes" in group:
                for attr in group["attributes"]:
                    if "id" in attr:  # Check if id exists
                        attr_name = attr["id"].replace(".", "_").upper()
                        attr_value = f"{group['id']}.{attr['id']}"
                        lines.append(f"    pub const {attr_name}: &str = \"{attr_value}\";")
            
            lines.append("}")
            lines.append("")
    
    return "\n".join(lines)

def generate_span_builders_rs(conventions):
    """Generate span builders for each group."""
    lines = [
        "// Generated span builders from SwarmSH v2 semantic conventions",
        "",
        "use opentelemetry::global::BoxedSpan;",
        "use opentelemetry::trace::Tracer;",
        "",
        "// Core span builders expected by lib.rs",
        "pub fn agent_lifecycle_span(tracer: &impl Tracer) -> BoxedSpan {",
        "    tracer.start(\"swarmsh.agent.lifecycle\")",
        "}",
        "",
        "pub fn work_coordination_span(tracer: &impl Tracer) -> BoxedSpan {",
        "    tracer.start(\"swarmsh.work.coordination\")",
        "}",
        "",
        "pub fn coordination_protocol_span(tracer: &impl Tracer) -> BoxedSpan {",
        "    tracer.start(\"swarmsh.coordination.protocol\")",
        "}",
        "",
        "// Generated span builders for all groups",
    ]
    
    for file_path, data in conventions.items():
        if "groups" not in data:
            continue
            
        for group in data["groups"]:
            if group.get("type") == "span":
                group_name = group["id"].replace(".", "_")
                function_name = f"create_{group_name}_span"
                
                lines.append(f"pub fn {function_name}(tracer: &impl Tracer) -> BoxedSpan {{")
                lines.append(f"    tracer.start(\"{group['id']}\")")
                lines.append("}")
                lines.append("")
    
    return "\n".join(lines)

def main():
    """Generate code from semantic conventions."""
    print("Generating SwarmSH v2 telemetry code from semantic conventions...")
    
    # Load conventions
    conventions = load_semantic_conventions()
    print(f"Loaded {len(conventions)} convention files")
    
    # Create output directory
    output_dir = Path("src/generated")
    output_dir.mkdir(exist_ok=True)
    
    # Generate attributes
    attributes_code = generate_attributes_rs(conventions)
    with open(output_dir / "attributes.rs", "w") as f:
        f.write(attributes_code)
    print("Generated attributes.rs")
    
    # Generate span builders
    span_builders_code = generate_span_builders_rs(conventions)
    with open(output_dir / "span_builders.rs", "w") as f:
        f.write(span_builders_code)
    print("Generated span_builders.rs")
    
    print("Code generation complete!")

if __name__ == "__main__":
    main()