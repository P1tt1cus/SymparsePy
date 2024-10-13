import argparse
from symparsepy import parse_pdb, search_symbols


def main():
    # Set up argument parsing
    parser = argparse.ArgumentParser(
        description='Search for symbols in a PDB file.')
    parser.add_argument('pdb_file', type=str, help='Path to the PDB file')
    parser.add_argument('--search', type=str,
                        help='Regex pattern to search for specific symbols')
    args = parser.parse_args()

    if args.search:
        # Search for symbols using regex
        symbols = search_symbols(args.pdb_file, args.search)
    else:
        # Load the entire PDB file
        symbols = parse_pdb(args.pdb_file)

    # Print the found symbols
    for symbol in symbols:
        print(symbol)


if __name__ == "__main__":
    main()
