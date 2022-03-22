#include <iostream>
#include <vector>
#include <string>
#include <filesystem>

using std::cout;
using std::string;
using std::to_string;

using std::endl;

using std::filesystem::directory_iterator;
using std::filesystem::directory_entry;
using std::filesystem::current_path;
using std::filesystem::path;
using std::filesystem::exists;

string red = "\x1b[31m";
string yellow = "\x1b[33m";
string reset = "\x1b[0m";

string fileSizeToString(int fileSize) {
    if (fileSize < 1000) {
        return to_string(fileSize) + "B";
    } else if ((fileSize / 1000) < 1000) {
        return to_string(fileSize / 1000) + "KB";
    } else if ((fileSize / 1000000) < 1000) {
        return to_string(fileSize / 1000000) + "MB";
    } else {
        return to_string(fileSize / 1000000000) + "GB";
    }
}

void listDirectory(string pathName) {
for (const directory_entry &file : directory_iterator(pathName)) {
        path filePath = file.path();
        if (file.is_directory()) {
            cout << yellow << filePath.filename().u8string() << " ";
        } else {
            cout << red << "(" << filePath.filename().u8string() << " - " << fileSizeToString(file.file_size()) << ") ";
        }
    }
}

bool filePathValid(string pathName) {
    return exists(pathName);
}

int main(int argc, char* argv[]) {
    string pathName;
    if (argc > 1) {
        pathName = argv[1];
    } else {
        pathName = current_path().u8string();
    }

    if (filePathValid(pathName)) listDirectory(pathName);
    else cout << red << "The provided path is not valid." << endl;

    cout << reset;
    return 0;
}