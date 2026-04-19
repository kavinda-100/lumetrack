#!/bin/bash

ROOT_DIR=$(pwd)
LOG_FILE="$ROOT_DIR/telemetry.log"

# --- Configuration ---
SERVICES=(
  "gateway"
  "telemetry"
  "discovery"
  "order-manager"
)

TOTAL_SERVICES=${#SERVICES[@]}
STARTED_COUNT=0
PIDS=()

cleanup() {
  echo -e "\n\n🛑 Shutting down all services..."
  for pid in "${PIDS[@]}"; do
    kill "$pid" 2>/dev/null
  done
  echo "✅ All services stopped."
  exit
}

trap cleanup SIGINT

echo "🚀 Starting LumeTrack Microservices ($TOTAL_SERVICES services total)..."

# 1. Start Redis
if ! redis-cli ping > /dev/null 2>&1; then
    echo "📦 Starting Redis server..."
    redis-server --daemonize yes
fi

# 2. Add Timestamp Header to Log File
echo -e "\n==========================================" >> "$LOG_FILE"
echo "  SESSION START: $(date '+%Y-%m-%d %H:%M:%S')" >> "$LOG_FILE"
echo -e "==========================================\n" >> "$LOG_FILE"

# 3. Start each service
for service in "${SERVICES[@]}"; do
  echo "📡 Attempting to start: $service..."
  
  if [ -d "$ROOT_DIR/$service" ]; then
    cd "$ROOT_DIR/$service" || continue
    
    if [ "$service" == "order-manager" ]; then
      bun run dev > /dev/null 2>&1 &
    elif [ "$service" == "telemetry" ]; then
      # Use RUST_LOG=info to ensure tracing calls are captured
      # We use 'unbuffer' or 'stdbuf' if you want to see them instantly in the file
      RUST_LOG=info cargo run --quiet >> "$LOG_FILE" 2>&1 &
      echo "📝 Telemetry is logging to $LOG_FILE (RUST_LOG=info)"
    else
      cargo run --quiet > /dev/null 2>&1 &
    fi
    
    PIDS+=($!)
    ((STARTED_COUNT++))
    echo "✅ [$STARTED_COUNT/$TOTAL_SERVICES] $service is initializing..."
    cd "$ROOT_DIR"
  else
    echo "❌ [$STARTED_COUNT/$TOTAL_SERVICES] FAILED: Directory '$service' not found."
  fi
  
  sleep 1.5 
done

# 3. Final Comparison
echo -e "\n--- Startup Report ---"
if [ "$STARTED_COUNT" -eq "$TOTAL_SERVICES" ]; then
    echo "🚀 Success: All $TOTAL_SERVICES services started successfully."
    echo "📝 Telemetry logs are being written to: $LOG_FILE"
else
    echo "⚠️ Warning: Only $STARTED_COUNT out of $TOTAL_SERVICES services started."
fi
echo "--- Total Services: $TOTAL_SERVICES | Started: $STARTED_COUNT | Failed: $((TOTAL_SERVICES - STARTED_COUNT)) ---"
echo "----------------------"
echo "Press Ctrl+C to stop everything."

wait