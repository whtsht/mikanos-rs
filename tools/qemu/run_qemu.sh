#!/bin/sh -ex

if [ $# -lt 1 ]
then
    echo "Usage: $0 <.efi file> [another file]"
    exit 1
fi

DEVENV_DIR=$(dirname "$0")
EFI_FILE=$1
ANOTHER_FILE=$2
DISK_IMG=$DEVENV_DIR/disk.img
MOUNT_POINT=$DEVENV_DIR/mnt

$DEVENV_DIR/make_image.sh $DISK_IMG $MOUNT_POINT $EFI_FILE $ANOTHER_FILE
$DEVENV_DIR/run_image.sh $DISK_IMG

rm -rf $DISK_IMG
rm -rf $MOUNT_POINT
