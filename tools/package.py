#!/usr/bin/env python3
"""Package the flat project with the pinned game's shared .hmod writer."""
import argparse
import hashlib
import json
from pathlib import Path
import horizon


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--horizon', type=Path, default=horizon.ROOT.parent/'horizon')
    args = parser.parse_args()
    with horizon.lock(horizon.ROOT/'target/stage.lock'):
        horizon.trait_stage(args.horizon.resolve(), horizon.ROOT/'target/trait')
        out = horizon.ROOT/'target/horizon-mod'/f"{horizon.MANIFEST['id']}.hmod"
        print(json.dumps(dict(file=str(out), size=out.stat().st_size,
                              sha256=hashlib.sha256(out.read_bytes()).hexdigest()), indent=2))


if __name__ == '__main__':
    main()
