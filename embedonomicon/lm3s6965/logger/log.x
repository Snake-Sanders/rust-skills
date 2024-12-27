SECTIONS
{
  // INFO keeps the .log ection in .ELF files
  // but does not deploys it on the target
  // 0 is the sart address
  .log 0 (INFO) : {
    *(.log);
  }
}

