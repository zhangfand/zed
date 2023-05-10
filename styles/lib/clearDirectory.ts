import fs from 'fs';
import path from 'path';

export function clearDirectory(directoryPath: string, directoriesToKeep: string[] = []): boolean {
  let success = true;
  if (fs.existsSync(directoryPath)) {
    fs.readdirSync(directoryPath).forEach((file) => {
      const curPath = path.join(directoryPath, file);

      // Check if the current path is a directory
      if (fs.lstatSync(curPath).isDirectory()) {
        // If it's a directory to keep, skip deleting its content
        if (directoriesToKeep.includes(file)) {
          clearDirectory(curPath, directoriesToKeep);
        } else {
          // It's a directory to delete, delete it recursively
          if (clearDirectory(curPath, directoriesToKeep)) {
            fs.rmdirSync(curPath);
          } else {
            success = false;
          }
        }
      } else {
        // It's a file, delete it
        try {
          fs.unlinkSync(curPath);
        } catch (e) {
          console.error(`Failed to delete file: ${curPath}`);
          success = false;
        }
      }
    });
    if (success) {
      console.log(`Cleared directory: ${directoryPath}`);
    } else {
      console.error(`Failed to clear directory: ${directoryPath}`);
    }
  } else {
    console.log(`Directory not found: ${directoryPath}`);
    success = false;
  }
  return success;
}
