#!/bin/bash

# Performance Benchmark Script for PDF to CBZ Converter
# Tests with large PDF file: Adler (Integrale 1).pdf (850MB, 270 pages)

set -e

PDF_FILE="samples/Adler (Integrale 1).pdf"
OUTPUT_DIR="/tmp/benchmark_results"
CLI="./src-cli/target/release/pdf-to-cbz"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "========================================"
echo "   PDF to CBZ Converter - BENCHMARK"
echo "========================================"
echo ""
echo "📄 Test File: Adler (Integrale 1).pdf"
echo "📊 Size: 850 MB"
echo "📖 Pages: 270"
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"
rm -f "$OUTPUT_DIR"/*.cbz 2>/dev/null || true

# Function to measure conversion time
benchmark() {
    local test_name="$1"
    local output_file="$2"
    shift 2
    local args="$@"
    
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}Test: $test_name${NC}"
    echo "Command: $CLI pdf-to-cbz \"$PDF_FILE\" -o \"$output_file\" $args"
    echo ""
    
    local start_time=$(date +%s)
    
    $CLI pdf-to-cbz "$PDF_FILE" -o "$output_file" $args
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    local minutes=$((duration / 60))
    local seconds=$((duration % 60))
    
    local file_size=$(ls -lh "$output_file" | awk '{print $5}')
    
    echo ""
    echo -e "${GREEN}✓ Completed in: ${minutes}m ${seconds}s${NC}"
    echo -e "${GREEN}✓ Output size: $file_size${NC}"
    echo ""
    
    # Store results
    echo "$test_name|${minutes}m ${seconds}s|$file_size" >> "$OUTPUT_DIR/results.txt"
}

# Clear previous results
rm -f "$OUTPUT_DIR/results.txt"

echo "Starting benchmark tests..."
echo ""
sleep 2

# Test 1: Lossless mode (fastest, original quality)
benchmark "1. Lossless Mode" \
    "$OUTPUT_DIR/adler_lossless.cbz" \
    "--lossless"

# Test 2: Low quality, low DPI (smallest file, fastest render)
benchmark "2. Low Quality (DPI 150, Q50)" \
    "$OUTPUT_DIR/adler_low.cbz" \
    "--dpi 150 --quality 50"

# Test 3: Standard quality (default)
benchmark "3. Standard (DPI 300, Q90)" \
    "$OUTPUT_DIR/adler_standard.cbz" \
    "--dpi 300 --quality 90"

# Test 4: Medium quality optimized
benchmark "4. Medium (DPI 200, Q85)" \
    "$OUTPUT_DIR/adler_medium.cbz" \
    "--dpi 200 --quality 85"

# Test 5: High quality (slow but great quality)
benchmark "5. High Quality (DPI 300, Q100)" \
    "$OUTPUT_DIR/adler_high.cbz" \
    "--dpi 300 --quality 100"

# Display results table
echo ""
echo "========================================"
echo "         BENCHMARK RESULTS"
echo "========================================"
echo ""
printf "%-35s %-15s %-15s\n" "Test" "Time" "Output Size"
echo "────────────────────────────────────────────────────────────────"

while IFS='|' read -r test time size; do
    printf "%-35s %-15s %-15s\n" "$test" "$time" "$size"
done < "$OUTPUT_DIR/results.txt"

echo ""
echo "========================================"
echo "         PERFORMANCE ANALYSIS"
echo "========================================"
echo ""

# Calculate speed (pages per minute)
echo "Pages processed per minute:"
while IFS='|' read -r test time size; do
    # Extract total seconds from time
    minutes=$(echo $time | sed 's/m.*//')
    seconds=$(echo $time | sed 's/.*m \(.*\)s/\1/')
    total_seconds=$((minutes * 60 + seconds))
    
    if [ $total_seconds -gt 0 ]; then
        pages_per_min=$(echo "scale=2; 270 * 60 / $total_seconds" | bc)
        printf "  %-35s %.2f pages/min\n" "$test:" "$pages_per_min"
    fi
done < "$OUTPUT_DIR/results.txt"

echo ""
echo "Output files saved in: $OUTPUT_DIR"
echo ""
echo "✓ Benchmark completed!"
