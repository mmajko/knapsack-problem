CONTENT=""
for FILENAME in "$@"
do
    CONTENT+="$(cat "$FILENAME" | awk -F " " '{print $1","$2","$3}')\n"
done

echo "knap_id,item_count,price"
echo "$CONTENT"