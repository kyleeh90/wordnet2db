# wordnet2db

Take Princeton's WordNet 3.1 files and outputs an SQLite database, SQL statements for importing, or a JSON file containing English words and definitions.

Files available from Princeton University at https://wordnet.princeton.edu/download/current-version

## Options
-c, --char-counts [<CHAR_COUNTS>...]
          Comma seperated list of character counts to save e.g. 4,7
-d, --directory <DIRECTORY>
          Directory where WordNet files are located (index.adj, index.adv, data.adj, etc)
-S, --dump-sql
          Renders database as SQL statements rather than an SQLite database
-k, --keep-numbers
          Keep words with numbers
-M, --max-chars <MAX_CHARS>
          Maximum character count of a word to save (default: 45) [default: 45]
-m, --min-chars <MIN_CHARS>
          Minimum character count of a word to save (default: 0) [default: 0]
-W, --only-whole-words
          Only keep words without punctuation or spaces
-o, --output-directory <OUTPUT_DIRECTORY>
          Directory to place output file into (default: working directory)
-J, --to-json
          Renders dictionary as JSON rather than an SQLite database
-h, --help
          Print help (see more with '--help')
-V, --version
          Print version
