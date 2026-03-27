#pragma once
#include <string>
#include <vector>
#include <map>
#include <cmath>
#include <algorithm>

class Recommender {
public:
    void addTrack(const std::string& id, const std::vector<float>& embedding) {
        embeddings[id] = embedding;
    }

    void likeTrack(const std::string& id) {
        if (std::find(userLikes.begin(), userLikes.end(), id) == userLikes.end()) {
            userLikes.push_back(id);
        }
    }

    std::vector<std::string> recommend(size_t topN) {
        if (userLikes.empty() || embeddings.empty()) {
            return {};
        }

        size_t dim = embeddings.begin()->second.size();
        std::vector<float> userProfile(dim, 0.0f);
        int count = 0;

        for (const auto& id : userLikes) {
            if (embeddings.count(id)) {
                for (size_t i = 0; i < dim; ++i) {
                    userProfile[i] += embeddings.at(id)[i];
                }
                count++;
            }
        }

        if (count == 0) return {};
        for (size_t i = 0; i < dim; ++i) userProfile[i] /= static_cast<float>(count);

        std::vector<std::pair<std::string, float>> scores;
        for (const auto& [id, emb] : embeddings) {
            if (std::find(userLikes.begin(), userLikes.end(), id) == userLikes.end()) {
                scores.push_back({id, cosineSimilarity(userProfile, emb)});
            }
        }

        std::sort(scores.begin(), scores.end(), [](const auto& a, const auto& b) {
            return a.second > b.second;
        });

        std::vector<std::string> result;
        for (size_t i = 0; i < std::min(topN, scores.size()); ++i) {
            result.push_back(scores[i].first);
        }
        return result;
    }

private:
    static float cosineSimilarity(const std::vector<float>& a, const std::vector<float>& b) {
        if (a.size() != b.size() || a.empty()) return 0.0f;
        float dot = 0.0f, normA = 0.0f, normB = 0.0f;
        for (size_t i = 0; i < a.size(); ++i) {
            dot += a[i] * b[i];
            normA += a[i] * a[i];
            normB += b[i] * b[i];
        }
        if (normA == 0.0f || normB == 0.0f) return 0.0f;
        return dot / (std::sqrt(normA) * std::sqrt(normB));
    }

    std::map<std::string, std::vector<float>> embeddings;
    std::vector<std::string> userLikes;
};
