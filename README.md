### imgcmp
A command-line tool that compares two images and determines if they are essentially the same, even if they have undergone slight modifications such as resizing, exposure changes, saturation adjustments, or minor edits.

Uses `pHash` algorithm as described [here](https://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html). The DCT dimensions and the maximum allowed Hamming distance between the hashes are configurable.

### Example usage
```
$ imgcmp picture1.jpg picture1_modified.jpg
Pictures are the same

$ imgcmp picture1.jpg different_picture.jpg
Pictures are different
```
