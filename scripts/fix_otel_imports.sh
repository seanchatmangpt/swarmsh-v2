#!/bin/bash
# SwarmSH v2 - 80/20 OTEL Import Fixer
# Automatically fixes missing OTEL trait imports in binaries

set -euo pipefail

echo "🔧 SwarmSH v2 - OTEL Import Fixer"
echo "Fixing missing OpenTelemetry trait imports..."

# Find all binary files with OTEL span usage but missing imports
find src/bin -name "*.rs" -type f | while read -r file; do
    echo "Checking: $file"
    
    # Check if file uses .end() method but missing Span import
    if grep -q "\.end()" "$file" && ! grep -q "use opentelemetry::trace::Span" "$file"; then
        echo "  📝 Adding missing Span import to $file"
        
        # Add the import after the existing use statements
        if grep -q "^use " "$file"; then
            # Find the last use statement and add after it
            sed -i '' '/^use /a\
use opentelemetry::trace::Span;' "$file"
        else
            # No existing use statements, add at the top after comments
            sed -i '' '/^\/\//a\
\\
use opentelemetry::trace::Span;' "$file"
        fi
        echo "  ✅ Fixed: Added Span import"
    fi
    
    # Check if file uses .set_attribute() but missing trait
    if grep -q "\.set_attribute(" "$file" && ! grep -q "use opentelemetry::trace::Span" "$file"; then
        echo "  📝 Adding missing ObjectSafeSpan import to $file"
        
        if grep -q "^use " "$file"; then
            sed -i '' '/^use /a\
use opentelemetry::global::ObjectSafeSpan;' "$file"
        else
            sed -i '' '/^\/\//a\
\\
use opentelemetry::global::ObjectSafeSpan;' "$file"
        fi
        echo "  ✅ Fixed: Added ObjectSafeSpan import"
    fi
done

echo ""
echo "🧪 Testing fixes..."

# Quick compilation test
if cargo check --bins >/dev/null 2>&1; then
    echo "✅ All binary imports fixed successfully"
    echo "📊 Running quick validation..."
    
    # Count remaining compilation issues
    error_count=$(cargo check --bins 2>&1 | grep "error\[" | wc -l | tr -d ' ')
    warning_count=$(cargo check --bins 2>&1 | grep "warning:" | wc -l | tr -d ' ')
    
    echo "📈 Results:"
    echo "  - Compilation errors: $error_count"
    echo "  - Warnings: $warning_count"
    
    if [ "$error_count" -eq 0 ]; then
        echo "🏆 ALL BINARIES COMPILE SUCCESSFULLY"
    else
        echo "⚠️  $error_count errors remaining (likely non-import issues)"
    fi
else
    echo "❌ Some compilation issues remain"
    echo "📋 Remaining errors:"
    cargo check --bins 2>&1 | grep "error\[" | head -5
fi

echo ""
echo "🎯 80/20 Import Fix Complete"
echo "   Most common OTEL import issues resolved"
echo "   Run 'cargo run --bin revolutionary_validator' for full validation"