#!/usr/bin/env bash

# This script runs `diesel migration redo` and then modifies `schema.rs` to add
# the suffix 'Mapping' to all enum typenames.
# This is the default suffix for diesel-derive-enum typenames.
# Without doing this, either the pg typename or the Rust typename have to be
# non-idiomatic, since the default Diesel typenames generated by diesel-cli are
# Pascal-cased versions of the pg typenames, which is the idiomatic Rust
# typename, and the Rust and Diesel typenames must differ since the latter is
# generated in the same scope as the former by a derive macro.
# This only really works for single migrations, where all the types are defined
# in one .sql file.

# Find the filename for the single migration being applied
tmp_filename=.tmp_sql_1
diesel migration redo 2>&1 | tee $tmp_filename
final_line=( $(tail -n 1 $tmp_filename) )
start_of_final_line=${final_line[@]:0:2}
if [ "$start_of_final_line" == 'Running migration' ]
then
  migration_version=${final_line[@]:2:1}
fi

# Find the typenames in the relevant .sql file
grep -i 'CREATE TYPE' migrations/${migration_version}/up.sql > $tmp_filename
enum_typenames=()
while read p; do
  words=($p)
  enum_typenames+=(${words[@]:2:1})
done < $tmp_filename
for typename in "${enum_typenames[@]}"; do
  capitalized_typename="${typename^}"
  sed -i -- "s/${capitalized_typename}/${capitalized_typename}Mapping/g" src/db/schema.rs
done

rm $tmp_filename
