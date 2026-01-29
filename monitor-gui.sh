#!/bin/bash

# Script pour tester la conversion GUI avec logs détaillés

echo "🔍 Monitoring PDF to CBZ Converter logs..."
echo "=========================================="
echo ""
echo "1. L'application est lancée"
echo "2. Ouvrez l'interface graphique"
echo "3. Chargez un fichier PDF"
echo "4. Cliquez sur 'Convert'"
echo "5. Les logs apparaîtront ci-dessous"
echo ""
echo "Logs en temps réel :"
echo "=========================================="

# Monitor les logs du processus
tail -f /dev/null &
PID=$!

# Chercher le processus de l'app
while true; do
    ps aux | grep -i "pdf-to-cbz-converter" | grep -v grep | grep -v tail | grep -v "$$"
    sleep 2
done
