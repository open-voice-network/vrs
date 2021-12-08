# Copyright 2021 The Open Voice Network
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

FROM rust:slim
RUN apt-get update && apt-get install -y \
build-essential checkinstall zlib1g-dev -y
COPY . /build-vrs/
RUN cd /build-vrs/ && rustup default nightly && rustup update && cargo build --release
LABEL Name=VRS Version=0.0.1
EXPOSE 8000
ENV ROCKET_ENV=stage
CMD ["/build-vrs/target/release/vrs"]
