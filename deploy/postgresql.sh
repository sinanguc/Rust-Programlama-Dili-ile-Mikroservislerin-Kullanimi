set -e

until diesel database setup; do
  >&2 echo "Postgresql çalışmıyor - servis durdu"
  sleep 1
done

>&2 echo "Postgresql çalışıyor - komutları işliyor"
diesel migration run