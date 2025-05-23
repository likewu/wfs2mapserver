#pragma once
#include "rust/cxx.h"
#include <memory>

namespace org {
namespace opencv {

struct MultiBuf;
struct BlobMetadata;

class OpencvClient {
public:
  OpencvClient();
  uint64_t put(MultiBuf &buf) const;
  void tag(uint64_t blobid, rust::Str tag) const;
  BlobMetadata metadata(uint64_t blobid) const;

  uint64_t maincv(uint64_t argc, rust::Vec<rust::Str> argv) const;

private:
  class impl;
  std::shared_ptr<impl> impl;
};

std::unique_ptr<OpencvClient> new_blobstore_client();

} // namespace opencv
} // namespace org
