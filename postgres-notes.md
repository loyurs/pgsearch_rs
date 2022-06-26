```

UPDATE searchtable SET tokens = setweight(to_tsvector('simple', $1), 'A') || setweight(to_tsvector('simple', $2), 'B') WHERE tokens = $3
```
