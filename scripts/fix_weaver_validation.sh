#!/bin/bash
# SwarmSH v2 - 80/20 OTEL Weaver Validation Fixer
# Fixes common semantic convention validation issues

set -euo pipefail

echo "🔧 SwarmSH v2 - OTEL Weaver Validation Fixer"
echo "Fixing semantic convention validation issues..."

# Check if weaver is available
if ! command -v weaver &> /dev/null; then
    echo "❌ OTEL Weaver not found. Install with:"
    echo "   cargo install weaver_forge"
    exit 1
fi

echo ""
echo "📋 Current validation status:"
cd semantic-conventions

# Run validation and capture output
validation_output=$(weaver validate . 2>&1 || true)
echo "$validation_output"

# Count errors
error_count=$(echo "$validation_output" | grep -c "Error" || echo "0")
echo ""
echo "📊 Found $error_count validation errors"

if [ "$error_count" -eq 0 ]; then
    echo "✅ All semantic conventions are valid!"
    exit 0
fi

echo ""
echo "🔧 Applying common fixes..."

# Fix 1: Remove invalid XPath expressions (common issue)
echo "  📝 Fixing XPath validation issues..."
find . -name "*.yaml" -type f | while read -r file; do
    if grep -q "XPath.*``" "$file" 2>/dev/null; then
        echo "    Fixing empty XPath in $file"
        sed -i '' 's/XPath.*``.*$//' "$file"
    fi
done

# Fix 2: Fix common YAML syntax issues
echo "  📝 Fixing YAML syntax issues..."
find . -name "*.yaml" -type f | while read -r file; do
    # Fix common indentation issues
    if grep -q "^[[:space:]]*-[[:space:]]*$" "$file" 2>/dev/null; then
        echo "    Fixing empty list items in $file"
        sed -i '' '/^[[:space:]]*-[[:space:]]*$/d' "$file"
    fi
    
    # Fix trailing spaces (common YAML issue)
    if grep -q "[[:space:]]$" "$file" 2>/dev/null; then
        echo "    Removing trailing spaces in $file"
        sed -i '' 's/[[:space:]]*$//' "$file"
    fi
done

# Fix 3: Ensure required fields are present
echo "  📝 Checking required fields..."
find . -name "*.yaml" -type f | while read -r file; do
    if grep -q "groups:" "$file" && ! grep -q "brief:" "$file"; then
        echo "    Adding missing brief field to $file"
        sed -i '' '/groups:/i\
brief: "Auto-generated brief for SwarmSH coordination"' "$file"
    fi
done

echo ""
echo "🧪 Re-running validation..."
new_validation_output=$(weaver validate . 2>&1 || true)
new_error_count=$(echo "$new_validation_output" | grep -c "Error" || echo "0")

echo "📊 Results:"
echo "  - Before: $error_count errors"
echo "  - After: $new_error_count errors"

if [ "$new_error_count" -eq 0 ]; then
    echo "🏆 ALL VALIDATION ERRORS FIXED!"
    echo ""
    echo "✅ Ready to generate code with:"
    echo "   weaver generate --template rust ."
elif [ "$new_error_count" -lt "$error_count" ]; then
    echo "✅ Reduced errors from $error_count to $new_error_count"
    echo ""
    echo "📋 Remaining issues to fix manually:"
    echo "$new_validation_output" | grep "Error" | head -3
else
    echo "⚠️  Manual intervention required"
    echo "📋 Common issues that need manual fixes:"
    echo "$new_validation_output" | grep "Error" | head -5
fi

cd ..
echo ""
echo "🎯 80/20 Weaver Fix Complete"
echo "   Run 'weaver validate semantic-conventions/' to verify"